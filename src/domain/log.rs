use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub time_unix_nano: i64,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub trace_flags: Option<i16>,
    pub severity_text: Option<String>,
    pub severity_number: Option<i32>,
    pub body: Option<serde_json::Value>,
    pub resource_attributes: Option<serde_json::Value>,
    pub scope_attributes: Option<serde_json::Value>,
    pub log_attributes: Option<serde_json::Value>,
    pub service_name: String,
}
