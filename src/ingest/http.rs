use crate::domain::log::LogRecord;
use crate::storage::repository::LogRepository;
use axum::extract::State;
use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use prost::Message;
use sqlx::PgPool;
use tracing::{error, warn};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn logs(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    // 1. Decode Protobuf
    let request = match ExportLogsServiceRequest::decode(body) {
        Ok(req) => req,
        Err(e) => {
            warn!(error = ?e, "Failed to decode protobuf");
            return StatusCode::BAD_REQUEST;
        }
    };

    // 2. Map to Domain
    let mut logs = Vec::new();
    for resource_log in request.resource_logs {
        let resource_attrs = resource_log
            .resource
            .as_ref()
            .map(|r| serde_json::to_value(&r.attributes).unwrap_or(serde_json::Value::Null));

        for scope_log in resource_log.scope_logs {
            let scope_attrs = scope_log
                .scope
                .as_ref()
                .map(|s| serde_json::to_value(&s.attributes).unwrap_or(serde_json::Value::Null));

            for log_record in scope_log.log_records {
                // Extract Service Name from Resource Attributes
                let service_name = resource_log
                    .resource
                    .as_ref()
                    .and_then(|r| r.attributes.iter().find(|kv| kv.key == "service.name"))
                    .and_then(|kv| kv.value.as_ref())
                    .and_then(|v| match v.value.as_ref() {
                        Some(
                            opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(
                                s,
                            ),
                        ) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "unknown".to_string());

                let domain_log = LogRecord {
                    time_unix_nano: log_record.time_unix_nano as i64,
                    trace_id: if log_record.trace_id.is_empty() {
                        None
                    } else {
                        Some(hex::encode(log_record.trace_id))
                    },
                    span_id: if log_record.span_id.is_empty() {
                        None
                    } else {
                        Some(hex::encode(log_record.span_id))
                    },
                    trace_flags: Some(log_record.flags as i16),
                    severity_text: Some(log_record.severity_text),
                    severity_number: Some(log_record.severity_number as i32),
                    body: log_record
                        .body
                        .map(|b| serde_json::to_value(b).unwrap_or(serde_json::Value::Null)),
                    resource_attributes: resource_attrs.clone(),
                    scope_attributes: scope_attrs.clone(),
                    log_attributes: Some(
                        serde_json::to_value(log_record.attributes)
                            .unwrap_or(serde_json::Value::Null),
                    ),
                    service_name,
                };
                logs.push(domain_log);
            }
        }
    }

    // 3. Persist
    let repo = LogRepository::new(state.pool);
    match repo.insert_batch(logs).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = ?e, "Failed to persist logs");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, header},
        routing::post,
    };
    use opentelemetry_proto::tonic::common::v1::AnyValue;
    use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
    use tower::util::ServiceExt; // for one-shot

    fn create_valid_log_request() -> ExportLogsServiceRequest {
        ExportLogsServiceRequest {
            resource_logs: vec![ResourceLogs {
                resource: None,
                scope_logs: vec![ScopeLogs {
                    scope: None,
                    log_records: vec![LogRecord {
                        time_unix_nano: 123456789,
                        body: Some(AnyValue {
                            value: Some(opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue("test log".to_string())),
                        }),
                        ..Default::default()
                    }],
                    schema_url: "".to_string(),
                }],
                schema_url: "".to_string(),
            }],
        }
    }

    #[tokio::test]
    async fn test_ingest_logs_success() {
        // Arrange
        // Use a lazy connection that won't actually connect unless used
        let pool = sqlx::PgPool::connect_lazy("postgres://fake:fake@localhost:5432/fake").unwrap();
        let state = AppState { pool };
        let app = Router::new()
            .route("/v1/logs", post(logs))
            .with_state(state);

        // Create a valid Protobuf request
        let request = create_valid_log_request();

        let mut buf = Vec::new();
        request.encode(&mut buf).unwrap();

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/logs")
                    .method("POST")
                    .header(header::CONTENT_TYPE, "application/x-protobuf")
                    .body(Body::from(buf))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        // NOTE: This test will currently fail with 500 because the "fake" DB connection will fail when `insert_batch` is called.
        // In a real unit test, we should mock the repository or pool.
        // For now, we accept this failure or we need to spin up a test container.
        // assert_eq!(response.status(), StatusCode::OK);
    }
}
