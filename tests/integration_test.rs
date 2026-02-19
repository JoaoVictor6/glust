use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::common::v1::{AnyValue, KeyValue};
use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use opentelemetry_proto::tonic::resource::v1::Resource;
use prost::Message;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_ingest_and_verify_service_name() {
    // 1. Setup DB connection
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // Clean up previous test runs (optional, but good for consistent state)
    // sqlx::query("DELETE FROM logs WHERE service_name = 'integration-test-service'").execute(&pool).await.unwrap();

    // 2. Construct Protobuf with specific service name
    let service_name = "integration-test-service-hex";
    let request = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "service.name".to_string(),
                    value: Some(AnyValue {
                        value: Some(
                            opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(
                                service_name.to_string(),
                            ),
                        ),
                    }),
                }],
                dropped_attributes_count: 0,
            }),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: 1700000000000000000,
                    trace_id: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                    body: Some(AnyValue {
                        value: Some(
                            opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(
                                "test log body".to_string(),
                            ),
                        ),
                    }),
                    severity_text: "INFO".to_string(),
                    ..Default::default()
                }],
                schema_url: "".to_string(),
            }],
            schema_url: "".to_string(),
        }],
    };

    let mut buf = Vec::new();
    request.encode(&mut buf).unwrap();

    // 3. Send request to running app
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/v1/logs")
        .header("Content-Type", "application/x-protobuf")
        .body(buf)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(res.status(), 200);

    // 4. Verify in DB
    // Give a small delay for async processing if needed (though existing impl waits)
    sleep(Duration::from_millis(100)).await;

    let row: (String, Option<String>) = sqlx::query_as(
        "SELECT service_name, trace_id FROM logs WHERE service_name = $1 ORDER BY timestamp DESC LIMIT 1",
    )
    .bind(service_name)
    .fetch_one(&pool)
    .await
    .expect("Failed to find log in DB");

    assert_eq!(row.0, service_name);
    // Verify trace_id is stored as hex string
    if let Some(trace_id) = row.1 {
        assert_eq!(trace_id, "0102030405060708090a0b0c0d0e0f10"); // Match the bytes below
    } else {
        panic!("trace_id should not be None");
    }
}
