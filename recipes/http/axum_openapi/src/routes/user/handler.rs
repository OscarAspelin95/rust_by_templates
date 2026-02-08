use crate::schema::{PostUser, QueryUser, User};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use tracing::warn;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get users by optional query", body = Vec<User>),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(QueryUser)
)]
pub async fn get_handler(Query(query_user): Query<QueryUser>) -> Response {
    // Here, we'd query the database based on the query params.
    warn!(
        "Skipping filtering by {:?} since this is a mock endpoint!",
        query_user
    );

    // Pretend these are the filtered/queried users returned by the database.
    let users: Vec<User> = (0..10).map(|_| User::mock()).collect();

    (StatusCode::OK, Json(users)).into_response()
}

#[utoipa::path(
    post,
    path = "/user",
    responses(
        (status = 201, description = "User created", body = User),
    ),
    request_body = PostUser
)]
pub async fn post_handler(Json(post_user): Json<PostUser>) -> Response {
    // Here, we'd insert into the database. For now, we mock.
    warn!("Mocking user since this is a mock endpoint!");

    let now = Utc::now();
    let user = User {
        id: Uuid::now_v7(),
        user_name: post_user.user_name,
        email: post_user.email,
        created_at: now,
        updated_at: now,
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
