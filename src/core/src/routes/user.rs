use actix_web::{HttpRequest, HttpResponse, Scope, web, get};
use cicada_common::CicadaResponse;
use cicada_database::User;
use cicada_database::auth::login::AuthLogin;
use crate::middleware::auth::Auth;
use crate::routes::*;

pub fn register_service() -> Scope {

    web::scope("/user")
        .service(authenticated)

}

#[get("/auth")]
fn authenticated(req: HttpRequest, auth: Auth) -> HttpResponse {
    only_auth!(req, auth);
    json_response(CicadaResponse::Ok(json!({
        "user": auth.get_user()
    })))
}