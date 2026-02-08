mod user;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(description = "User API with OpenAPI documentation"))]
struct ApiDoc;

pub fn get_routes() -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(user::get_handler, user::post_handler))
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
