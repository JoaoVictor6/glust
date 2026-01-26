use axum::extract::State;
use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use prost::Message;
use sqlx::PgPool;
use tracing::warn;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn logs(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    // Attempt to decode the protobuf body
    match ExportLogsServiceRequest::decode(body) {
        Ok(_request) => {
            let _pool = state.pool;
            // Use ? to print the struct (Debug trait)
            // This is "json-like" and safe
            StatusCode::OK
        }
        Err(e) => {
            warn!(error = ?e, "Failed to decode protobuf");
            StatusCode::BAD_REQUEST
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
        assert_eq!(response.status(), StatusCode::OK);
    }
}
