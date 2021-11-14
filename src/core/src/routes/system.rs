use std::collections::HashMap;
use actix_web::{HttpResponse, Scope, web, get};
use actix_web::web::Data;
use tera::{Context, Tera};
use serde_json::json;
use cicada_common::{Cicada, EmailConfiguration};
use crate::routes::{empty_route, html_response};

pub fn register_service() -> Scope {

    web::scope("")
        .service(index)

}

#[get("/")]
fn index(cicada: Data<Cicada>, tera: Data<Tera>) -> HttpResponse {
    html_response(empty_route(), (tera.as_ref(), cicada.as_ref(), "index.j2"))
}

