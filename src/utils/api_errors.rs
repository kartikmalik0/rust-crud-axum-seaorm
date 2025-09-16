use axum::{ response::IntoResponse, Json };
use hyper::{ header, StatusCode };
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<i8>,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;

        // Build JSON body
        let body = Json(
            json!({
            "status_code": status_code.as_u16(), // Convert to number
            "error_code": self.error_code,
            "message": self.message
        })
        );

        // Combine status, headers, and body into a Response
        (status_code, [(header::CONTENT_TYPE, "application/json")], body).into_response()
    }
}
