use axum::{ extract::State, Extension, Json };
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::{ ActiveValue::Set, EntityTrait };

use crate::{
    models::{ post_models::CreatePostModel, user_models::AppState },
    utils::api_errors::APIError,
};

pub async fn create_post(
    State(state): State<AppState>,
    Extension(identity): Extension<entity::user::Model>,
    Json(post_data): Json<CreatePostModel>
) -> Result<(), APIError> {
    let db = &state.db;
    let post_entity = entity::post::ActiveModel {
        title: Set(post_data.title),
        text: Set(post_data.text),
        image: Set(post_data.image),
        created_at: Set(Utc::now().naive_local().to_string()),
        user_id: Set(identity.id),
        ..Default::default()
    };

    entity::post::Entity
        ::insert(post_entity)
        .exec(db).await
        .map_err(|e| APIError {
            message: "Failed to insert Post".to_string(),
            error_code: Some(1),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(())
}
