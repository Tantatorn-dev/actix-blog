use actix_web::{post, web, Responder, Result};

use crate::{
    db::postgres::Postgres,
    dtos::{
        auth::{Login, UserSession},
        user::User,
    },
};

#[post("/login")]
pub async fn login(info: web::Form<Login>) -> Result<impl Responder> {
    let _pg = Postgres::connect_to_db().await;

    let query_str = format!("SELECT * FROM blog_user WHERE username='{}'", info.username);

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

    if let Some(user) = user {
        if user.password == info.password {
            let query_str = format!(
                "INSERT INTO user_session (blog_user_id) VALUES ('{}') RETURNING token",
                user.id
            );

            let row = _pg.client.query(query_str.as_str(), &[]).await.unwrap();

            let user_session = row
                .iter()
                .map(|row| UserSession {
                    token: row.get(0),
                })
                .collect::<Vec<UserSession>>()
                .pop().unwrap();

                Ok(web::Json(user_session.token))
        } else {
            Ok(web::Json("Invalid password".to_string()))
        }
    } else {
        Ok(web::Json("User not found".to_string()))
    }
}
