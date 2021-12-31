mod system;

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::ServiceConfig;
use serde_json::json;
use serde_json::Value::Null;
use tera::{Context, Tera};
use cicada_common::{Cicada, CicadaError, CicadaResponse, SystemConfiguration};

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

    let config_lock = tera.1.config.lock().unwrap();
    let system = config_lock.get("system").unwrap().as_any().downcast_ref::<SystemConfiguration>().unwrap();

    let mut context = Context::new();
    context.insert("data", &data.unwrap());
    context.insert("version", &tera.1.version);
    context.insert("system_name", &system.name);

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

fn json_response(data: CicadaResponse) -> HttpResponse {

    if let Err(error) = data {
        return error_response(error);
    }

    HttpResponseBuilder::new(StatusCode::OK)
        .json(json!({
            "status": StatusCode::OK.as_u16(),
            "success": true,
            "message": StatusCode::OK.canonical_reason(),
            "data": data.unwrap()
        }))

}

fn error_response(error: CicadaError) -> HttpResponse {

    let status_code = match StatusCode::from_u16(error.code) {
        Ok(code) => code,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    };

    error!("{}", error.message);

    HttpResponseBuilder::new(status_code)
        .content_type("application/json")
        .json(json!({
            "status": status_code.as_u16(),
            "success": false,
            "message": status_code.canonical_reason(),
            "data": json!(Null)
        }))

}

#[cfg(test)]
mod test {

    use actix_web::http::StatusCode;
    use serde_json::Value;
    use crate::routes::empty_route;

    #[test]
    fn test_empty_route_data() {
        assert!(empty_route().is_ok());
        assert_eq!(empty_route().unwrap(), Value::Null);
    }

}