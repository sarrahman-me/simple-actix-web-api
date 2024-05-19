use actix_web::{
    delete, get, patch, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[derive(Serialize, Deserialize)]
struct BookRes {
    id: u32,
    title: String,
    author: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseType<T> {
    message: String,
    status: u16,
    data: T,
}

type BookDb = Arc<Mutex<HashMap<u32, Book>>>;

/**
 * Menambahkan data buku baru
 */
#[post("/book")]
async fn add_book(payload: web::Json<Book>, db: web::Data<BookDb>) -> impl Responder {
    // inisialisasi db
    let mut db = db.lock().unwrap();

    let title = &payload.title;
    let author = &payload.author;

    // cek apakah sudah ada judul buku yang sama
    if db.values().any(|book: &Book| book.title == *title) {
        return HttpResponse::BadRequest().json(ResponseType {
            message: format!("Title {title} sudah pernah ditambahkan"),
            status: 400,
            data: None::<BookRes>,
        });
    }

    // buat id unik
    let id_unik: u32 = db.keys().max().unwrap_or(&0) + 1;

    // tambahkan ke dalam db
    db.insert(
        id_unik,
        Book {
            title: title.to_owned(),
            author: author.to_owned(),
        },
    );

    // mengembalikan response data yang baru ditambahkan
    HttpResponse::Created().json(ResponseType {
        message: String::from("Berhasil menambahkan buku baru"),
        status: 201,
        data: BookRes {
            id: id_unik,
            title: title.to_owned(),
            author: author.to_owned(),
        },
    })
}

/**
 * Mendapatkan semua data buku
 */
#[get("/book")]
async fn find_all(db: web::Data<BookDb>) -> impl Responder {
    let db = db.lock().unwrap();

    // membuat format Book menjadi BookRes
    let all_book: Vec<BookRes> = db
        .iter()
        .map(|(&id, book)| BookRes {
            id,
            title: book.title.clone(),
            author: book.author.clone(),
        })
        .collect();

    // Mengembalikan semua buku
    HttpResponse::Ok().json(ResponseType {
        message: String::from("Berhasil mendapatkan semua buku"),
        status: 200,
        data: all_book,
    })
}

/**
 * Mendapatkan buku dengan id
 */
#[get("/book/{id}")]
async fn find(id: web::Path<u32>, db: web::Data<BookDb>) -> impl Responder {
    let db = db.lock().unwrap();
    let id_book = id.into_inner();

    if !db.contains_key(&id_book) {
        return HttpResponse::NotFound().json(ResponseType {
            message: String::from("Buku tidak ditemukan"),
            status: 400,
            data: None::<BookRes>,
        });
    }

    let book = db.get(&id_book).unwrap();

    HttpResponse::Ok().json(ResponseType {
        message: String::from("Berhasil mendapatkan data buku"),
        status: 200,
        data: Some(BookRes {
            id: id_book,
            title: book.title.to_owned(),
            author: book.author.to_owned(),
        }),
    })
}

/**
 * Mengupdate data buku
 */
#[patch("/book/{id}")]
async fn update(
    id: web::Path<u32>,
    payload: web::Json<Book>,
    db: web::Data<BookDb>,
) -> impl Responder {
    let mut db = db.lock().unwrap();
    let id = id.into_inner();

    if !db.contains_key(&id) {
        return HttpResponse::BadRequest().json(ResponseType {
            message: String::from("Data buku tidak ada di system"),
            status: 400,
            data: None::<Book>,
        });
    }

    db.insert(id, payload.into_inner());

    let updated_data = db.get(&id).unwrap();

    HttpResponse::Ok().json(ResponseType {
        message: String::from("Berhasil mengupdate data buku"),
        status: 200,
        data: Some(BookRes {
            id,
            title: updated_data.title.to_owned(),
            author: updated_data.author.to_owned(),
        }),
    })
}

/**
 * Menghapus buku dengan id
 */
#[delete("/book/{id}")]
async fn delete_book(id: web::Path<u32>, db: web::Data<BookDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let id_book = id.into_inner();

    if !db.contains_key(&id_book) {
        return HttpResponse::NotFound().json(ResponseType {
            message: format!("Buku dengan id {id_book} tidak ditemukan"),
            status: 404,
            data: None::<Book>,
        });
    }

    db.remove(&id_book);

    HttpResponse::Accepted().json(ResponseType {
        message: format!("Buku dengan id {id_book} berhasil dihapus"),
        status: 202,
        data: None::<Book>,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server berjalan di port {port}");

    let book_db: BookDb = Arc::new(Mutex::new(HashMap::<u32, Book>::new()));

    HttpServer::new(move || {
        let app_data: web::Data<BookDb> = web::Data::new(book_db.clone());

        App::new()
            .app_data(app_data)
            .service(add_book)
            .service(find)
            .service(find_all)
            .service(update)
            .service(delete_book)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
