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

    // Run migrations
    match storage::db::Database::run_migrations(&pool).await {
        Ok(_) => println!("Migrations ran successfully!"),
        Err(e) => {
            tracing::error!("Failed to run migrations: {:?}", e);
            // Optionally exit or continue based on severity. For now, we log and continue (or panic).
            // panic!("Database migration failed");
        }
    }

    // Setup router
    let app = axum::Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .route("/v1/logs", axum::routing::post(ingest::http::logs))
        .with_state(ingest::http::AppState { pool });

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
