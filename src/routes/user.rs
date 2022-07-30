use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct User {
    id: u32,
    username: String,
    password: String,
}

#[get("/user")]
pub async fn list_user() -> Result<impl Responder> {
    let user = User {
        id: 0,
        username: "admin".to_string(),
        password: "admin".to_string(),
    };
    let users = vec![user.clone(), user.clone(), user.clone()];
    Ok(web::Json(users))
}

#[get("/user/{id}")]
pub async fn get_user_by_id(info: web::Path<u32>) -> Result<impl Responder> {
    let user = User {
	id: info.into_inner(),
	username: "admin".to_string(),
	password: "admin".to_string(),
    };
    Ok(web::Json(user))
}
