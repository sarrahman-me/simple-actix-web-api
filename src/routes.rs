use actix_web::web;
use crate::handlers::{add_book, find, find_all, update, delete_book};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_book);
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete_book);
}
