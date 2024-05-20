use crate::models::{Book, BookRes, ResponseType};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type BookDb = Arc<Mutex<HashMap<u32, Book>>>;

#[post("/book")]
pub async fn add_book(payload: web::Json<Book>, db: web::Data<BookDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let title = &payload.title;
    let author = &payload.author;

    if db.values().any(|book: &Book| book.title == *title) {
        return HttpResponse::BadRequest().json(ResponseType {
            message: format!("Title {title} sudah pernah ditambahkan"),
            status: 400,
            data: None::<BookRes>,
        });
    }

    let id_unik: u32 = db.keys().max().unwrap_or(&0) + 1;

    db.insert(
        id_unik,
        Book {
            title: title.to_owned(),
            author: author.to_owned(),
        },
    );

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

#[get("/book")]
pub async fn find_all(db: web::Data<BookDb>) -> impl Responder {
    let db = db.lock().unwrap();
    let all_book: Vec<BookRes> = db
        .iter()
        .map(|(&id, book)| BookRes {
            id,
            title: book.title.to_owned(),
            author: book.author.to_owned(),
        })
        .collect();

    HttpResponse::Ok().json(ResponseType {
        message: String::from("Berhasil mendapatkan semua buku"),
        status: 200,
        data: all_book,
    })
}

#[get("/book/{id}")]
pub async fn find(id: web::Path<u32>, db: web::Data<BookDb>) -> impl Responder {
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

#[patch("/book/{id}")]
pub async fn update(
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

    let updated_data: &Book = db.get(&id).unwrap();

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

#[delete("/book/{id}")]
pub async fn delete_book(id: web::Path<u32>, db: web::Data<BookDb>) -> impl Responder {
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
