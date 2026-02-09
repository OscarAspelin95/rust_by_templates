use tokio;
use tower_http::cors::CorsLayer;
mod environment;
mod errors;
use crate::errors::AppError;
use axum;
use environment::Environment;
mod routes;
use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
mod schema;

fn setup_tracing() {
    let formatting_layer = BunyanFormattingLayer::new("axum_tracing".into(), std::io::stdout)
        .skip_fields(vec!["file".to_string(), "line".to_string()].into_iter())
        .expect("Cannot to skip some formatting fields");
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    setup_tracing();

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
