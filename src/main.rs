use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    let posts = vec![
        Post {
            id: 1,
            title: String::from("First Post"),
            content: String::from("This is my first post content"),
        },
        Post {
            id: 2,
            title: String::from("Second Post"),
            content: String::from("This is my second post content"),
        },
    ];

    HttpResponse::Ok().json(posts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_posts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}