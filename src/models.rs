use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct BookRes {
    pub id: u32,
    pub title: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseType<T> {
    pub message: String,
    pub status: u16,
    pub data: T,
}
