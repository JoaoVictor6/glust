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
    let otel_log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // Setup Tracer
    let tracer_provider = init_tracer_provider()?;
    let tracer = tracer_provider.tracer("test-client");
    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_file(true);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,opentelemetry=debug".into());

    tracing_subscriber::registry()
        .with(otel_trace_layer) // Add trace layer first
        .with(otel_log_layer)
        .with(console_layer)
        .with(env_filter)
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

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;

fn init_tracer_provider() -> Result<SdkTracerProvider, Box<dyn std::error::Error>> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:4318/v1/traces") // Standard collector port
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .build();

    opentelemetry::global::set_tracer_provider(provider.clone());
    Ok(provider)
}

use opentelemetry_sdk::Resource;

fn init_logger_provider() -> Result<SdkLoggerProvider, Box<dyn std::error::Error>> {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:3000/v1/logs")
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .build()?;

    let resource = Resource::builder()
        .with_service_name("test_client_service")
        .build();

    let processor = BatchLogProcessor::builder(exporter).build();

    Ok(SdkLoggerProvider::builder()
        .with_resource(resource)
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

#[tracing::instrument(skip(params))]
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
