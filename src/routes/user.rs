use actix_web::{get, web, Responder, Result};
use serde::{Serialize};

use crate::db::postgres::{Postgres};

#[derive(Serialize, Clone, Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
}

#[get("/user")]
pub async fn list_user() -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let rows = _pg
        .client
        .query("SELECT * FROM blog_user", &[])
        .await
        .unwrap();

    let users = rows
        .iter()
        .map(|row| User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
        })
        .collect::<Vec<User>>();
    
    println!("{:?}", users);

    Ok(web::Json(users))
}

#[get("/user/{id}")]
pub async fn get_user_by_id(info: web::Path<i32>) -> Result<impl Responder> {
    let user = User {
        id: info.into_inner(),
        username: "admin".to_string(),
        password: "admin".to_string(),
    };
    Ok(web::Json(user))
}
