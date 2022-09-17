use actix_web::{get, post, web, Responder, Result};

use crate::{
    db::postgres::Postgres,
    dtos::user::{NewUser, User},
};

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

    Ok(web::Json(users))
}

#[get("/user/{id}")]
pub async fn get_user_by_id(info: web::Path<i32>) -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let id = info.into_inner();

    let query_str = format!("SELECT * FROM blog_user WHERE id={}", id);

    let row = _pg.client.query(query_str.as_str(), &[]).await.unwrap();

    let user = row
        .iter()
        .map(|row| User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
        })
        .collect::<Vec<User>>()
        .pop();

    Ok(web::Json(user))
}

#[post("/user")]
pub async fn new_user(info: web::Form<NewUser>) -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let query_str = format!(
        "INSERT INTO blog_user (username, password) VALUES ('{}', '{}')",
        info.username, info.password
    );

    match _pg.client.query(query_str.as_str(), &[]).await {
        Ok(_) => Ok(web::Json("User created successfully")),
        Err(_) => Ok(web::Json("Create user failed")),
    }
}
