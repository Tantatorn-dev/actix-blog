use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blog {
    pub id: i32,
    pub blog_user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewBlog {
    pub blog_user_id: i32,
    pub title: String,
    pub content: String,
}
