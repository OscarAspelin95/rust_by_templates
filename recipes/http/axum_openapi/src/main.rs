use tokio;
use tower_http::cors::CorsLayer;
mod environment;
mod errors;
use crate::errors::AppError;
use axum;
use environment::Environment;
use tracing::info;
mod routes;
mod schema;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt().init();

    info!("Checking environment...");
    let env = Environment::new();
    let address = env.socket_address()?;

    info!("Starting server...");
    let listener = tokio::net::TcpListener::bind(address).await?;
    let router = axum::Router::new()
        .merge(routes::get_routes())
        .layer(CorsLayer::permissive());

    axum::serve(listener, router).await?;
    Ok(())
}
