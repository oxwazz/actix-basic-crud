use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::error::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello oxwazz!")
}

#[derive(Deserialize)]
struct Info {
    username: Option<String>,
}

#[get("/users/{user_id}/{friend}")]
async fn dynamic(path: web::Path<(u32, String)>, info: web::Query<Info>) -> Result<String, Box<dyn Error>> {
    let (user_id, friend) = path.into_inner();
    let tes = info.username.as_deref().unwrap_or("");
    Ok(format!("{} {} {}", user_id, friend, tes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(dynamic)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}