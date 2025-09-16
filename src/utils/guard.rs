use axum::{ extract::{ Request, State }, middleware::Next, response::Response };
use hyper::StatusCode;
use sea_orm::{ EntityTrait, QueryFilter, ColumnTrait };

use crate::models::user_models::AppState;
use crate::utils::{ api_errors::APIError, jwt::decode_jwt };

pub async fn guard(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next
) -> Result<Response, APIError> {
    let token_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(APIError {
            message: "Token Not Found".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(1),
        })?;

    // strip "Bearer " prefix if present
    let token = token_header
        .strip_prefix("Bearer ")
        .or_else(|| token_header.strip_prefix("bearer "))
        .unwrap_or(token_header);

    println!("token ---- {}", token);

    let claims = decode_jwt(token).map_err(|_| APIError {
        message: "Token Not Found".to_string(), // Fixed typo: "Fount" -> "Found"
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(1),
    })?;

    println!("claims {:?}", claims);

    // Access the database from app_state
    let db = &app_state.db;

    let identity = entity::user::Entity
        ::find()
        .filter(entity::user::Column::Email.eq(claims.email.to_lowercase()))
        .one(db).await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(1),
        })?
        .ok_or(APIError {
            message: "UNAUTHORIZED".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(1),
        })?;

    // Insert the user identity into request extensions
    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
