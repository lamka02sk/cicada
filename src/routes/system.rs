use actix_web::{Scope, web};

pub fn configure() -> Scope {
    web::scope("/")
}