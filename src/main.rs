use actix_web::{App, HttpResponse, HttpServer, web, Result};
use actix_web::http::StatusCode;
use cicada_common::{Cicada, Configuration};

mod routes;

fn main() {

    println!("{:?}", cicada_common::Cicada::new());

}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//
//
//
//     HttpServer::new(|| {
//         App::new()
//             .service(routes::system())
//             .default_service(
//                 web::route().to(not_found)
//             )
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
//
// async fn not_found() -> Result<HttpResponse> {
//     Ok(HttpResponse::build(StatusCode::NOT_FOUND)
//         .content_type("text/html; charset=utf-8")
//         .body("<h1>Error 404</h1>"))
// }
