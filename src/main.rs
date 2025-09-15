use std::net::SocketAddr;
use chrono::Utc;
use sea_orm::{ ActiveValue::Set, Database, DatabaseConnection, ActiveModelTrait };
use tokio::net::TcpListener;
use axum::{ extract::State, response::IntoResponse, routing::get, Router };
use uuid::Uuid;
use tracing::{ info, error };
use entity::user;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}
// Store your connection string as a constant or environment variable
const DATABASE_URL: &str =
    "postgresql://postgres.ibqtyeaigdalewzpgzkq:abc123456.23add@aws-1-ap-south-1.pooler.supabase.com:5432/postgres?pool_size=20";

#[tokio::main]
async fn main() {
    //connect to the db and store the connection in a struct globla state
    let db = match Database::connect(DATABASE_URL).await {
        Ok(db) => {
            info!("Database connected Successfully");
            db
        }
        Err(err) => {
            error!("Database connection failed: {:?}", err);
            panic!("Failed to connect to database: {err}");
        }
    };

    let app_state: AppState = AppState { db };

    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/create-user", get(create_user))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hey there"
}

async fn create_user(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;

    let user_model = user::ActiveModel {
        name: Set("test".to_owned()),
        email: Set(format!("test_{}@gmail.com", Uuid::new_v4())), // Make email unique
        password: Set("12345678".to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    match user_model.insert(db).await {
        Ok(user) => {
            info!("User created successfully with UUID: {}", user.uuid);
            format!("User created with UUID: {}", user.uuid)
        }
        Err(e) => {
            error!("Error creating user: {:?}", e);
            format!("Error creating user: {}", e)
        }
    }
}
