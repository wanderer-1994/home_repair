use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckPayload {
    pub status: String,
}

/// Service health check handler
pub async fn health_check() -> (StatusCode, Json<HealthCheckPayload>) {
    (
        StatusCode::OK,
        Json(HealthCheckPayload {
            status: String::from("Ok"),
        }),
    )
}
