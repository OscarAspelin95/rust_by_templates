use std::net::SocketAddr;

use tokio::{self};
use tower_http::cors::CorsLayer;
mod environment;
mod errors;
use crate::errors::AppError;
use axum;
use environment::Environment;
mod routes;
use tracing::{Level, info};
use tracing_subscriber::{self, util::SubscriberInitExt};
mod middleware;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(Level::INFO)
        .finish()
        .init();

    info!("Checking environment...");
    let env = Environment::new();
    let address = env.socket_address()?;

    info!("Starting server...");
    let listener = tokio::net::TcpListener::bind(address).await?;
    let router = axum::Router::new()
        .merge(routes::get_routes())
        .layer(CorsLayer::permissive());

    // NOTE - we need ".into_make_service_with_connect_info" for tower-governor to work properly.
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
