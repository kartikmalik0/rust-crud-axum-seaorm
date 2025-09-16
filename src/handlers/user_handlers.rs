use axum::{ extract::{ Path, State }, http::StatusCode, response::{ IntoResponse }, Json };
use sea_orm::{ ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter };
use uuid::Uuid;

use crate::{
    models::user_models::{ AppState, UpdateUserModel, UserModel },
    utils::api_errors::APIError,
};

pub async fn update_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> Result<impl IntoResponse, APIError> {
    let db = &state.db;

    let user = entity::user::Entity
        ::find()
        .filter(entity::user::Column::Email.eq(uuid))
        .one(db).await
        .map_err(|e| {
            // turn DB error into APIError
            APIError {
                message: format!("Database error while finding user: {}", e),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: Some(1),
            }
        })?
        // early return if error
        .ok_or(APIError {
            // return 404 if no user found
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(2),
        })?;

    let mut active_user: entity::user::ActiveModel = user.into();
    active_user.name = Set(user_data.name);
    active_user.update(db).await.map_err(|e| {
        APIError {
            message: format!("Failed to update user: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(3),
        }
    })?;

    Ok((
        StatusCode::OK,
        Json(
            serde_json::json!({
             "message": "User updated successfully",
            "uuid": uuid
        })
        ),
    ))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>
) -> Result<impl IntoResponse, APIError> {
    let db = &state.db;

    let user = entity::user::Entity
        ::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(db).await
        .map_err(|e| APIError {
            message: format!("Database error while finding user: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(1),
        })
        ? // early return if DB error
        .ok_or(APIError {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(2),
        })?;

    entity::user::Entity
        ::delete_by_id(user.id)
        .exec(db).await
        .map_err(|e| APIError {
            message: format!("Failed to delete user: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(3),
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message":"User deleted successfully", "uuid":uuid})),
    ))
}

pub async fn get_all_users(State(state): State<AppState>) -> Result<impl IntoResponse, APIError> {
    let db = &state.db;

    let users = entity::user::Entity
        ::find()
        .all(db).await
        .map_err(|e| APIError {
            message: format!("Database error while fetching all users: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(1),
        })?;

    let user_models: Vec<UserModel> = users
        .into_iter()
        .map(|user| UserModel {
            name: user.name,
            email: user.email,
            password: user.password,
            uuid: user.uuid,
            created_at: user.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(user_models)))
}
