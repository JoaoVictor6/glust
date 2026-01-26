use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::common::v1::{any_value, AnyValue, KeyValue};
use opentelemetry_proto::tonic::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use opentelemetry_proto::tonic::resource::v1::Resource;
use prost::Message;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let payload = create_otlp_logs_payload();
    let encoded = payload.encode_to_vec();

    let mut file = File::create("payload.bin").expect("Failed to create payload file");
    file.write_all(&encoded)
        .expect("Failed to write payload to file");

    println!("Generated OTLP payload: {} bytes", encoded.len());
    println!("Saved to: payload.bin");
}

fn create_otlp_logs_payload() -> ExportLogsServiceRequest {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let log_record = LogRecord {
        time_unix_nano: now,
        observed_time_unix_nano: now,
        severity_number: 9, // INFO
        severity_text: "INFO".to_string(),
        body: Some(AnyValue {
            value: Some(any_value::Value::StringValue(
                "Stress test log message for OTLP ingestion benchmark".to_string(),
            )),
        }),
        attributes: vec![
            KeyValue {
                key: "service.name".to_string(),
                value: Some(AnyValue {
                    value: Some(any_value::Value::StringValue("stress-test".to_string())),
                }),
            },
            KeyValue {
                key: "environment".to_string(),
                value: Some(AnyValue {
                    value: Some(any_value::Value::StringValue("benchmark".to_string())),
                }),
            },
            KeyValue {
                key: "request.id".to_string(),
                value: Some(AnyValue {
                    value: Some(any_value::Value::StringValue(
                        "550e8400-e29b-41d4-a716-446655440000".to_string(),
                    )),
                }),
            },
        ],
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: vec![0u8; 16],
        span_id: vec![0u8; 8],
    };

    let scope_logs = ScopeLogs {
        scope: None,
        log_records: vec![log_record],
        schema_url: String::new(),
    };

    let resource_logs = ResourceLogs {
        resource: Some(Resource {
            attributes: vec![
                KeyValue {
                    key: "service.name".to_string(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue("glust-stress-test".to_string())),
                    }),
                },
                KeyValue {
                    key: "service.version".to_string(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue("1.0.0".to_string())),
                    }),
                },
            ],
            dropped_attributes_count: 0,
        }),
        scope_logs: vec![scope_logs],
        schema_url: String::new(),
    };

    ExportLogsServiceRequest {
        resource_logs: vec![resource_logs],
    }
}
