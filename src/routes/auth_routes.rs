use axum::{ http::method, routing::post, Router };
use tower_http::cors::{ Any, CorsLayer };
use crate::{ handlers::auth_handlers, models::user_models::AppState };
pub fn auth_routes() -> Router<AppState> {
    let cors = CorsLayer::new().allow_methods([method::Method::POST]).allow_origin(Any);

    Router::new()
        .route("/create-user", post(auth_handlers::create_user))
        .route("/get-user", post(auth_handlers::get_user))
        .layer(cors)
}
