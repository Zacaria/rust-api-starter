use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    post,
    path = "/api/process",
    request_body = String,
    responses(
        (status = 200, description = "Processed successfully")
    )
)]
pub async fn process(path: String) -> impl IntoResponse {
    tracing::info!("processing {}", path);
    StatusCode::OK
}
