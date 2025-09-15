use axum::{ http::Method, routing::{ delete, get, post, put }, Router };
use tower_http::cors::{ Any, CorsLayer };
use crate::{ handlers::user_handlers, models::user_models::AppState };

pub fn user_routes() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    Router::new()
        .route("/", get(user_handlers::get_all_users))
        .route("/{uuid}", put(user_handlers::update_user))
        .route("/{uuid}", delete(user_handlers::delete_user))
        .layer(cors)
}
