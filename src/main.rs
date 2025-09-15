use std::net::SocketAddr;
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    Condition,
    ColumnTrait,
    Database,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
};
use tokio::net::TcpListener;
use axum::{
    extract::{ Path, State },
    response::{ IntoResponse, Response },
    routing::{ delete, get, post, put },
    Json,
    Router,
};
use uuid::Uuid;
use tracing::{ info, error };
use entity::user;
use serde_json::json;

use crate::models::user_models::{ CreateUserModel, GetUserModel, UpdateUserModel, UserModel };

mod models;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

// Store your connection string as a constant or environment variable
const DATABASE_URL: &str =
    "postgresql://postgres.ibqtyeaigdalewzpgzkq:abc123456.23add@aws-1-ap-south-1.pooler.supabase.com:5432/postgres?pool_size=20";

#[tokio::main]
async fn main() {
    // Connect to the DB and store the connection in app state
    let db = match Database::connect(DATABASE_URL).await {
        Ok(db) => {
            info!("Database connected Successfully");
            db
        }
        Err(err) => {
            error!("Database connection failed: {:?}", err);
            panic!("Failed to connect to database: {:?}", err);
        }
    };

    let app_state: AppState = AppState { db };

    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/create-user", post(create_user))
        .route("/get-user", post(get_user))
        .route("/get-users", get(get_all_users))
        .route("/update-user/{uuid}", put(update_user))
        .route("/delete-user/{uuid}", delete(delete_user))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hey there"
}

async fn create_user(
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

async fn get_user(State(state): State<AppState>, Json(user_data): Json<GetUserModel>) -> Response {
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

async fn update_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> impl IntoResponse {
    let db = &state.db;

    let user = entity::user::Entity
        ::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(db).await
        .unwrap();

    if let Some(user) = user {
        let mut active_user: entity::user::ActiveModel = user.into();
        active_user.name = Set(user_data.name);

        active_user.update(db).await.unwrap();
        (StatusCode::ACCEPTED, "Updated")
    } else {
        (StatusCode::NOT_FOUND, "User Not found")
    }
}

async fn delete_user(State(state): State<AppState>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let db = &state.db;

    let user = entity::user::Entity
        ::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(db).await
        .unwrap();

    if let Some(user) = user {
        entity::user::Entity::delete_by_id(user.id).exec(db).await.unwrap();
        (StatusCode::ACCEPTED, "User Deleted")
    } else {
        (StatusCode::NOT_FOUND, "User not found")
    }
}

// Alternative - using a generic approach with serde_json::Value
async fn get_all_users(State(state): State<AppState>) -> Response {
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
