use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
    age: Option<u32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/query")]
async fn with_params(info: web::Query<Info>) -> impl Responder {
    let response = format!(
        "Hello {}! {}",
        info.name,
        info.age.map_or(String::from(""), |age| format!("You are {} years old.", age))
    );
    HttpResponse::Ok().body(response)
}

async fn manual_something() -> impl Responder {
    HttpResponse::Ok().body("Hey there, this is a manual something!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(with_params)
            .route("/hey", web::get().to(manual_hello))
            .route("/something", web::get().to(manual_something))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}