use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json,
};
use sea_orm::{ ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter };
use tracing::{ error, info };
use uuid::Uuid;

use crate::models::user_models::{ AppState, ErrorResponse, UpdateUserModel, UserModel };

pub async fn update_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> impl IntoResponse {
    let db = &state.db;

    match entity::user::Entity::find().filter(entity::user::Column::Uuid.eq(uuid)).one(db).await {
        Ok(Some(user)) => {
            let mut active_user: entity::user::ActiveModel = user.into();
            active_user.name = Set(user_data.name);

            match active_user.update(db).await {
                Ok(_) => (StatusCode::OK, "Updated").into_response(),
                Err(e) => {
                    error!("Failed to update user: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Failed to update user".to_string(),
                        }),
                    ).into_response()
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            error!("Database error while finding user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            ).into_response()
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>
) -> impl IntoResponse {
    let db = &state.db;

    match entity::user::Entity::find().filter(entity::user::Column::Uuid.eq(uuid)).one(db).await {
        Ok(Some(user)) => {
            match entity::user::Entity::delete_by_id(user.id).exec(db).await {
                Ok(_) => (StatusCode::OK, "User deleted").into_response(),
                Err(e) => {
                    error!("Failed to delete user: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Failed to delete user".to_string(),
                        }),
                    ).into_response()
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            error!("Database error while finding user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            ).into_response()
        }
    }
}

pub async fn get_all_users(State(state): State<AppState>) -> Response {
    let db = &state.db;

    match entity::user::Entity::find().all(db).await {
        Ok(users) => {
            info!("Retrieved {} users", users.len());

            // Convert users to UserModel for consistent response structure
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

            (StatusCode::OK, Json(user_models)).into_response()
        }
        Err(e) => {
            error!("Database error while fetching all users: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            ).into_response()
        }
    }
}
