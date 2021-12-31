use actix_web::{HttpResponse, Scope, web, get};
use actix_web::web::Data;
use tera::Tera;
use serde_json::Value;
use cicada_common::Cicada;
use cicada_database::ConnectionPool;
use crate::routes::{empty_route, html_response, json_response};

pub fn register_service() -> Scope {

    web::scope("")
        .service(index)
        .service(ping)
        .service(status)

}

#[get("/")]
fn index(cicada: Data<Cicada>, tera: Data<Tera>) -> HttpResponse {
    html_response(empty_route(), (tera.as_ref(), cicada.as_ref(), "index.j2"))
}

#[get("/ping")]
fn ping() -> HttpResponse {
    json_response(Ok(Value::Null))
}

#[get("/status")]
fn status(db: Data<ConnectionPool>) -> HttpResponse {
    json_response(cicada_system::get_status(db.as_ref()))
}
