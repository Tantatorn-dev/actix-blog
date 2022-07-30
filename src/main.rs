use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use db::postgres::connect_to_db;

mod routes;
mod db;
use crate::routes::user::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _client = connect_to_db().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(list_user)
            .service(get_user_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
