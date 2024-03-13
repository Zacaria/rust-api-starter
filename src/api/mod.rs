use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use self::middleware::log_request_info::AddRequestInfo;

pub mod errors;
pub mod health;
pub mod middleware;
pub mod process;

#[derive(OpenApi)]
#[openapi(paths(process::process, health::health))]
struct ApiDoc;

pub fn app() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api/doc").url("/api/doc/swagger.json", ApiDoc::openapi()))
        .route("/api/process", post(process::process))
        .route("/health", get(health::health))
        .layer(AddRequestInfo)
        .layer(axum::middleware::from_fn(
            middleware::context::context_resolver,
        ))
}
