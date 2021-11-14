mod routes;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use cicada_common::{Cicada, SystemConfiguration};
use tera::Tera;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    // Tera templates
    let tera = match Tera::new("templates/**/*.j2") {
        Ok(tera) => tera,
        Err(error) => {
            panic!("Could not compile Tera templates: {}", error);
        }
    };

    // Configuration
    let configuration = Cicada::new();
    let configuration = configuration.config.lock();

    if let Err(error) = configuration {
        panic!("Could not lock the application data for http server setup: {}", error);
    }

    let configuration = configuration.unwrap();
    let system_config: &SystemConfiguration = configuration.get("system").unwrap().as_any().downcast_ref().unwrap();

    // Server startup
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Cicada::new()))
            .app_data(Data::new(tera.clone()))
            .configure(routes::configure)
    });

    for bind in &system_config.bind {
        server = server.bind((bind.to_owned(), system_config.port))?;
    }

    server = server.server_hostname(system_config.hostname.to_owned());

    if system_config.workers > 0 {
        server = server.workers(system_config.workers);
    }

    server.run().await

}
