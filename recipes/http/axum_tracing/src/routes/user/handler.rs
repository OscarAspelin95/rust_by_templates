use std::time::Duration;

use crate::schema::{PostUser, User};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::{info, instrument};
use uuid::Uuid;

/// NOTE - we need to be careful about what we trace (e.g., passwords and other personal information).
#[instrument(fields(req_id = %Uuid::now_v7(), post_user))]
pub async fn post_handler(Json(post_user): Json<PostUser>) -> Response {
    info!("Mocking inserting user into database...");
    let mut user = User::mock();
    user.user_name = post_user.user_name;
    user.email = post_user.email;
    std::thread::sleep(Duration::from_millis(200));
    info!("Db insert completed successfully!");

    (StatusCode::CREATED, Json(user)).into_response()
}

/// NOTE - we need to be careful about what we trace (e.g., passwords and other personal information).
#[instrument(fields(req_id = %Uuid::now_v7()))]
pub async fn get_handler() -> Response {
    info!("Mocking querying database...");
    let user = User::mock();
    std::thread::sleep(Duration::from_millis(200));
    info!("Db query completed successfully!");

    (StatusCode::OK, Json(user)).into_response()
}
