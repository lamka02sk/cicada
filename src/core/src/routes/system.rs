use actix_web::{HttpResponse, Scope, web, get, post, HttpRequest};
use actix_web::web::Data;
use tera::Tera;
use serde_json::Value;
use cicada_common::Cicada;
use cicada_database::{ConnectionPool, NewUser};
use cicada_database::auth::login_form::LoginForm;
use cicada_database::auth::login::AuthLogin;
use cicada_database::User;
use crate::extractors::Headers;
use crate::middleware::auth::Auth;
use crate::routes::*;

pub fn register_service() -> Scope {

    web::scope("/")
        .service(index)
        .service(ping)
        .service(status)
        .service(create_admin_account)
        .service(login)
        .service(check_login)
        .service(logout)

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
    json_response(cicada_controllers::get_status(db.as_ref()))
}

#[post("/setup/create-admin-account")]
fn create_admin_account(req: HttpRequest, db: Data<ConnectionPool>, auth: Auth, mut user: web::Json<NewUser>) -> HttpResponse {
    not_auth!(req, auth);
    json_response(cicada_controllers::create_admin_account(db.as_ref(), &mut user))
}

#[post("/auth/login")]
fn login(req: HttpRequest, headers: Headers, auth: Auth, db: Data<ConnectionPool>, mut login: web::Json<LoginForm>) -> HttpResponse {
    not_auth!(req, auth);
    json_response(cicada_controllers::auth::login(headers.into(), db.as_ref(), &mut login))
}

#[get("/auth/check")]
fn check_login(req: HttpRequest, auth: Auth) -> HttpResponse {
    only_auth!(req, auth);
    json_response(empty_route())
}

#[get("/auth/logout")]
fn logout(req: HttpRequest, auth: Auth, db: Data<ConnectionPool>) -> HttpResponse {
    only_auth!(req, auth);
    json_response(cicada_controllers::auth::logout(db.as_ref(), auth.get_login().unwrap()))
}