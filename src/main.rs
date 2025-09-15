use std::net::SocketAddr;

use sea_orm::{ Database };
use tokio::net::TcpListener;
use axum::{ response::{ IntoResponse }, Router };
use tracing::{ info, error };

use crate::models::user_models::AppState;
use dotenv::dotenv;
use std::env;
mod models;
mod routes;
mod handlers;

// Store your connection string as a constant or environment variable

#[tokio::main]
async fn main() {
    dotenv().ok();

    let datbase_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the DB and store the connection in app state
    let db = match Database::connect(datbase_url).await {
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
