use actix_web::{post, web, Responder, Result};

use crate::{
    db::postgres::Postgres, dtos::auth::Login,
};

#[post("/login")]
pub async fn login(info: web::Form<Login>) -> Result<impl Responder> {
    let _ = Postgres::connect_to_db().await;

    Ok(web::Json(info.into_inner()))
}
