use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateUserModel {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[derive(serde::Serialize)]
pub struct GetUserResponseModel {
    pub token: String,
}
