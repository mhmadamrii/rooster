use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod db;
use crate::db::{AppState, init_db};
use std::sync::Mutex;

#[derive(Serialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!, this is rust app. Pretty cool innit?")
}

#[post("/posts")]
async fn create_post(data: web::Data<AppState>, post: web::Json<CreatePostRequest>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    
    match conn.execute(
        "INSERT INTO posts (title, content) VALUES (?1, ?2)",
        [&post.title, &post.content],
    ) {
        Ok(_) => {
            // Get the ID of the last inserted row
            let id = conn.last_insert_rowid();
            HttpResponse::Created().json(Post {
                id: id as i32,
                title: post.title.clone(),
                content: post.content.clone(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create post: {}", e)),
    }
}

#[get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    
    let mut stmt = match conn.prepare("SELECT id, title, content FROM posts") {
        Ok(stmt) => stmt,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let posts_result = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    });

    match posts_result {
        Ok(posts_iter) => {
            let posts: Result<Vec<Post>, _> = posts_iter.collect();
            match posts {
                Ok(posts) => HttpResponse::Ok().json(posts),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error collecting posts: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Query error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    
    // Initialize database connection
    let db_conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Ok(());
        }
    };

    // Create app state with database connection
    let app_state = web::Data::new(AppState {
        db: Mutex::new(db_conn),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(get_posts)
            .service(create_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}