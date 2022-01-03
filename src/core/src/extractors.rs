use std::future;
use std::future::ready;
use std::net::{IpAddr, Ipv4Addr};
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use cicada_database::CicadaHeaders;

#[derive(Debug, Deserialize)]
pub struct Headers {
    pub user_agent: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub auth_token: Option<String>
}

impl FromRequest for Headers {

    type Error = Error;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        let user_agent = get_header_value(req, "User-Agent");

        let ip_address = match req.peer_addr() {
            Some(value) => Some(value.ip()),
            None => None
        };

        let auth_token = match get_header_value(req, "Authorization") {
            Some(value) => match value.split(" ").last() {
                Some(value) => Some(value.to_string()),
                None => None
            },
            None => None
        };

        ready(Ok(Headers {
            user_agent,
            ip_address,
            auth_token
        }))

    }

}

impl Into<CicadaHeaders> for Headers {
    fn into(self) -> CicadaHeaders {
        CicadaHeaders {
            user_agent: match self.user_agent {
                Some(value) => value,
                None => String::new()
            },
            ip_address: match self.ip_address {
                Some(value) => value,
                None => IpAddr::V4(Ipv4Addr::UNSPECIFIED)
            }
        }
    }
}

fn get_header_value(req: &HttpRequest, name: &str) -> Option<String> {
    match req.headers().get(name) {
        Some(value) => match value.to_str() {
            Ok(value) => Some(value.to_string()),
            Err(error) => {
                error!("Could not extract '{}' header from request at '{}': {}", name, req.path(), error);
                None
            }
        },
        None => None
    }
}
