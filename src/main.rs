use crate::models::Book;
use crate::routes::init_routes;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod handlers;
mod models;
mod routes;

type BookDb = Arc<Mutex<HashMap<u32, Book>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server berjalan di port {port}");

    let book_db: BookDb = Arc::new(Mutex::new(HashMap::<u32, Book>::new()));

    HttpServer::new(move || {
        let app_data: web::Data<BookDb> = web::Data::new(book_db.clone());
        App::new().app_data(app_data).configure(init_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
