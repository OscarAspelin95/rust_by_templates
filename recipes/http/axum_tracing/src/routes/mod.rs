mod health;
mod user;
use axum::{Router, routing::get};

pub fn get_routes() -> Router {
    let health_router = Router::new().route("/", get(health::handler));
    let user_router = Router::new().route("/", get(user::get_handler).post(user::post_handler));
    Router::new()
        .nest("/health", health_router)
        .nest("/user", user_router)
}
