use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "OK")
    )
)]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
