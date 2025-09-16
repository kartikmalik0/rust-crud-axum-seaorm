use serde::{ Serialize, Deserialize };

pub struct PostModel {
    pub id: i32,
    pub text: String,
    pub user_id: i32,
    pub image: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostModel {
    pub text: String,
    pub image: String,
    pub title: String,
}
