use axum::{extract::Path, http::StatusCode, response::Json};

use crate::models::{ApiResponse, HealthResponse};

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
        message: "Server is running".to_string(),
    })
}

/// Simple hello endpoint
pub async fn hello() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Hello, World!".to_string()),
        message: "Hello endpoint".to_string(),
    })
}

/// Hello with name parameter
pub async fn hello_name(Path(name): Path<String>) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some(format!("Hello, {name}!")),
        message: "Hello endpoint with name".to_string(),
    })
}

/// 404 not found handler
pub async fn not_found() -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            success: false,
            data: None,
            message: "Route not found".to_string(),
        }),
    )
}
