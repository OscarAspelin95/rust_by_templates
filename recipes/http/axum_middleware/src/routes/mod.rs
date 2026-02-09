mod health;
use crate::middleware::log_middleware;
use axum::{Router, middleware, routing::get};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

/// Here, we add a logging middleware and a rate limiter to all our routes.
pub fn get_routes() -> Router {
    let rl_cfg = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(1)
        .finish()
        .expect("failed to generate rate limit config");

    let health_router = Router::new()
        .route("/", get(health::handler))
        .layer(middleware::from_fn(log_middleware))
        .layer(GovernorLayer::new(rl_cfg));

    Router::new().nest("/health", health_router)
}
