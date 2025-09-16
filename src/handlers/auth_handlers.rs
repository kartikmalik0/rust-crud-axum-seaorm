use crate::{
    models::user_models::{
        AppState,
        CreateUserModel,
        GetUserModel,
        GetUserResponseModel,
        UserModel,
    },
    utils::{ api_errors::APIError, jwt::encode_jwt },
};
use axum::{ extract::State, response::{ IntoResponse }, Json };
use chrono::Utc;
use entity::user;
use hyper::StatusCode;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    Condition,
    ColumnTrait,
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
) -> Result<Json<GetUserResponseModel>, APIError> {
    let db = &state.db;

    

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

    let token = encode_jwt(user.email).map_err(|r| APIError {
        message: "Failed to login".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(1),
    })?;

    Ok(Json(GetUserResponseModel { token }))
}
