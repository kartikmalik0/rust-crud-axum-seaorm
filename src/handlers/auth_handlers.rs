use crate::models::user_models::{ AppState, CreateUserModel, GetUserModel, UserModel };
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
) -> impl IntoResponse {
    let db = &state.db;

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    match user_model.insert(db).await {
        Ok(user) => {
            info!("User created successfully with UUID: {}", user.uuid);
            (StatusCode::CREATED, format!("User created with UUID: {}", user.uuid))
        }
        Err(e) => {
            error!("Error creating user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating user: {}", e))
        }
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Json(user_data): Json<GetUserModel>
) -> Response {
    let db = &state.db;

    let user = match
        user::Entity
            ::find()
            .filter(
                Condition::all()
                    .add(user::Column::Email.eq(user_data.email))
                    .add(user::Column::Password.eq(user_data.password))
            )
            .one(db).await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "User not found").into_response();
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
        }
    };

    let data = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    (StatusCode::OK, Json(data)).into_response()
}
