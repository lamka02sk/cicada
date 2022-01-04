use std::future::{Future, ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::{Error, FromRequest, HttpMessage};
use actix_web::web::Data;
use cicada_database::auth::login::AuthLogin;
use cicada_database::ConnectionPool;

pub struct AuthenticateMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticateMiddleware<S>
    where S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
          S::Future: 'static,
          B: 'static,
{

    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let auth_token = match req.headers().get("Authorization") {
            Some(value) => match value.to_str() {
                Ok(value) => match value.split(" ").last() {
                    Some(value) => Some(value.to_string()),
                    _ => None
                },
                _ => None
            },
            _ => None
        };

        if let (Some(db), Some(token)) = (req.app_data::<Data<ConnectionPool>>(), auth_token) {

            let auth_login = AuthLogin::from_token(&db.as_ref(), &token);

            if let Some(auth_login) = auth_login {
                req.extensions_mut()
                    .insert::<AuthLogin>(auth_login);
            }

        }

        let future = self.service.call(req);

        Box::pin(async move {
            Ok(future.await?)
        })

    }

}

pub struct AuthenticateMiddlewareFactory {}

impl AuthenticateMiddlewareFactory {
    pub fn new() -> Self {
        AuthenticateMiddlewareFactory {}
    }
}

impl<S, B> Transform<S> for AuthenticateMiddlewareFactory
    where S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
          S::Future: 'static,
          B: 'static {

    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticateMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateMiddleware {
            service,
        }))
    }

}

#[derive(Clone)]
pub struct Auth(Option<AuthLogin>);

impl Into<Option<AuthLogin>> for Auth {
    fn into(self) -> Option<AuthLogin> {
        self.0
    }
}

impl FromRequest for Auth {

    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(
            match req.extensions().get::<AuthLogin>() {
                Some(auth_login) => Ok(Auth(Some(auth_login.clone()))),
                None => Ok(Auth(None))
            }
        )
    }

}