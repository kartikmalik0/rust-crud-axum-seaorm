use std::net::SocketAddr;

use sea_orm::{ Database, DatabaseConnection };
use tokio::net::TcpListener;
use axum::{ response::{ IntoResponse }, routing::{ get }, Router };
use tracing::{ info, error };

use crate::models::user_models::AppState;

mod models;
mod routes;
mod handlers;

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
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes())
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hey there"
}
