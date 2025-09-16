use std::{ fmt::format, os::windows::io::IntoRawSocket };

use crate::{
    models::user_models::{ AppState, CreateUserModel, GetUserModel, UserModel },
    utils::api_errors::APIError,
};
use axum::{ extract::State, response::{ IntoResponse, Response }, Json };
use chrono::Utc;
use entity::user;
use hyper::StatusCode;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    Condition,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
};
use tracing::{ error, info };
use uuid::Uuid;

pub async fn create_user(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUserModel>
) -> Result<impl IntoResponse, APIError> {
    let db = &state.db;

    //check if user with this mail

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_model.insert(db).await.map_err(|e| {
        error!("Error creatating user: {:?}", e);
        APIError {
            message: format!("Error creating user: {}", e),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(3),
        }
    })?;

    info!("User created successfully with UUID: {}", user.uuid);

    Ok((
        StatusCode::CREATED,
        Json(
            serde_json::json!({
            "uuid": user.uuid,
            "message": "User created successfully"
        })
        ),
    ))
}

pub async fn get_user(
    State(state): State<AppState>,
    Json(user_data): Json<GetUserModel>
) -> Result<impl IntoResponse, APIError> {
    let db = &state.db;

    if
        let Some(_) = user::Entity
            ::find()
            .filter(user::Column::Email.eq(user_data.email.clone()))
            .one(db).await
            .map_err(|e| APIError {
                message: format!("DB error while checking user: {}", e),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: Some(1),
            })?
    {
        return Err(APIError {
            message: "User with this email already exists".to_string(),
            status_code: StatusCode::CONFLICT, // 409
            error_code: Some(2),
        });
    }

    let user = user::Entity
        ::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(user_data.email))
                .add(user::Column::Password.eq(user_data.password))
        )
        .one(db).await
        .map_err(|e| {
            // convert DB error into APIError automatically
            APIError {
                message: format!("DB error while fetching user: {}", e),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: Some(3),
            }
        })
        ? // `?` will return early if Err(APIError)
        .ok_or(APIError {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(4),
        })?;

    let data = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    Ok((StatusCode::OK, Json(data)))
}
