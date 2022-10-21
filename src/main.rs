use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use routes::{
    blog::{get_blog_by_id, list_blog, new_blog},
    user::{get_user_by_id, list_user, new_user}, auth::login,
};

mod db;
mod dtos;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(list_user)
            .service(get_user_by_id)
            .service(new_user)
            .service(list_blog)
            .service(get_blog_by_id)
            .service(new_blog)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
