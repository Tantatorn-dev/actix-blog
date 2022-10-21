use actix_web::{get, post, web, Responder, Result};

use crate::{
    db::postgres::Postgres,
    dtos::blog::{Blog, NewBlog},
};

#[get("/blog")]
pub async fn list_blog() -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let rows = _pg.client.query("SELECT * FROM blog", &[]).await.unwrap();

    let users = rows
        .iter()
        .map(|row| Blog {
            id: row.get(0),
            blog_user_id: row.get(1),
            title: row.get(2),
            content: row.get(3),
        })
        .collect::<Vec<Blog>>();

    Ok(web::Json(users))
}

#[get("/blog/{id}")]
pub async fn get_blog_by_id(info: web::Path<i32>) -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let id = info.into_inner();

    let query_str = format!("SELECT * FROM blog WHERE id={}", id);

    let row = _pg.client.query(query_str.as_str(), &[]).await.unwrap();

    let user = row
        .iter()
        .map(|row| Blog {
            id: row.get(0),
            blog_user_id: row.get(1),
            title: row.get(2),
            content: row.get(3),
        })
        .collect::<Vec<Blog>>()
        .pop();

    Ok(web::Json(user))
}

#[post("/blog")]
pub async fn new_blog(info: web::Form<NewBlog>) -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let query_str = format!(
        "INSERT INTO blog (blog_user_id, title, content) VALUES ('{}', '{}', '{}')",
        info.blog_user_id, info.title, info.content
    );

    match _pg.client.query(query_str.as_str(), &[]).await {
        Ok(_) => Ok(web::Json("Blog created successfully")),
        Err(_) => Ok(web::Json("Create blog failed")),
    }
}
