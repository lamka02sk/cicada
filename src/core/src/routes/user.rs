use actix_web::{HttpRequest, HttpResponse, Scope, web, get, put};
use actix_web::web::Data;
use cicada_common::CicadaResponse;
use cicada_database::{ConnectionPool, SelfUpdateUser, User};
use cicada_database::auth::login::{AuthLogin, UUIDAuthLogin};
use crate::middleware::auth::Auth;
use crate::routes::*;

pub fn register_service() -> Scope {

    web::scope("/user")
        .service(authenticated)
        .service(update_self)
        .service(logins)
        .service(disable_login)

}

#[get("/auth")]
fn authenticated(req: HttpRequest, auth: Auth) -> HttpResponse {
    only_auth!(req, auth);
    json_response(CicadaResponse::Ok(json!({
        "user": auth.get_user()
    })))
}

#[put("/update/self")]
fn update_self(req: HttpRequest, auth: Auth, db: Data<ConnectionPool>, user: web::Json<SelfUpdateUser>) -> HttpResponse {
    only_auth!(req, auth);
    json_response(cicada_controllers::users::update_self(db.as_ref(), &user))
}

#[get("/logins")]
fn logins(req: HttpRequest, auth: Auth, db: Data<ConnectionPool>) -> HttpResponse {
    only_auth!(req, auth);
    json_response(cicada_controllers::users::get_logins(db.as_ref(), &auth.get_user().unwrap()))
}

#[put("/login/disable")]
fn disable_login(req: HttpRequest, auth: Auth, db: Data<ConnectionPool>, login: web::Json<UUIDAuthLogin>) -> HttpResponse {
    only_auth!(req, auth);
    json_response(cicada_controllers::users::disable_login(db.as_ref(), &auth.get_user().unwrap(), &login))
}