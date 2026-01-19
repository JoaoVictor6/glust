mod adapter;
mod domain;
mod ingest;
mod storage;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database connection
    let pool = storage::db::Database::create_connection().await;
    println!("Successfully connected to the database!");

    // Setup router
    let app = axum::Router::new()
        .route("/v1/logs", axum::routing::post(ingest::http::logs))
        .with_state(ingest::http::AppState { pool });

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
