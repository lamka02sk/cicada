mod system;

use actix_web::client::SendRequestError::Http;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, ServiceConfig};
use serde_json::{json, Value};
use serde_json::Value::Null;
use tera::{Context, Tera};
use cicada_common::{Cicada, CicadaError, CicadaResponse};

pub fn configure(config: &mut ServiceConfig) {

    config
        .service(system::register_service());

}

fn empty_route() -> CicadaResponse {
    Ok(Null)
}

fn html_response(data: CicadaResponse, tera: (&Tera, &Cicada, &str)) -> HttpResponse {

    if let Err(error) = data {
        return error_response(error);
    }

    let mut context = Context::new();
    context.insert("data", &data.unwrap());

    let html = tera.0.render(tera.2, &context);

    if let Err(error) = html {
        return error_response(CicadaError {
            code: 500,
            message: error.to_string()
        })
    }

    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("text/html")
        .body(html.unwrap())

}

fn error_response(error: CicadaError) -> HttpResponse {

    let status_code = match StatusCode::from_u16(error.code) {
        Ok(code) => code,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    };

    HttpResponseBuilder::new(status_code)
        .content_type("application/json")
        .json(json!({
            "status": status_code.as_u16(),
            "success": false,
            "message": status_code.canonical_reason(),
            "data": json!(Null)
        }))

}
