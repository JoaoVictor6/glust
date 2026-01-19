use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::logs::{BatchLogProcessor, SdkLoggerProvider};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup OpenTelemetry Logger Provider
    let logger_provider = init_logger_provider()?;

    // 2. Setup Tracing Subscriber with OTEL bridge
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);
    let console_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_file(true);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(console_layer)
        .init();

    info!("Test client starting...");

    // 3. Setup Axum Router
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/generate", post(generate_logs));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("Test client listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_logger_provider() -> Result<SdkLoggerProvider, Box<dyn std::error::Error>> {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:3000/v1/logs")
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .build()?;

    let processor = BatchLogProcessor::builder(exporter).build();

    // Use Resource::new which takes IntoIterator<Item=KeyValue>
    // Wait, Resource::new is private?
    // Let's try Resource::default() assuming it implements Default.
    // If not, we found a really weird version issue.
    // Assuming Resource::default() works if using correct import/version.
    Ok(SdkLoggerProvider::builder()
        .with_log_processor(processor)
        .build())
}

async fn ping() -> impl IntoResponse {
    info!(target: "test_client", event = "ping_received", "Pong! This is a log from test-client.");
    "Pong"
}

#[derive(serde::Deserialize)]
struct GenerateParams {
    count: usize,
    message: String,
}

async fn generate_logs(Json(params): Json<GenerateParams>) -> impl IntoResponse {
    for i in 0..params.count {
        info!(
            target: "test_client",
            iteration = i,
            "Generated log: {} #{}", params.message, i
        );
    }
    format!("Generated {} logs", params.count)
}
