mod routes;

#[macro_use]
extern crate log;
extern crate simplelog;

use actix_cors::Cors;
use actix_web::{App, http, HttpServer};
use actix_web::web::Data;
use cicada_common::{Cicada, FileManager, SystemConfiguration, TextFile};
use tera::Tera;
use simplelog::*;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    // Configuration
    let configuration = Cicada::new();
    let configuration = configuration.config.lock();

    if let Err(error) = configuration {
        panic!("Could not lock the application data for http server setup: {}", error);
    }

    let configuration = configuration.unwrap();
    let system_config: &SystemConfiguration = configuration.get("system").unwrap().as_any().downcast_ref().unwrap();

    initialize_logging(&system_config);

    info!("{} is starting...", system_config.name);
    info!("Initializing HTML templating engine...");

    // Tera templates
    let tera = initialize_templating();

    info!("Preparing actix-web server...");

    let cors = system_config.cors.clone();

    // Server startup
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(get_cors_middleware(cors.clone()))
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

fn initialize_logging(system_config: &SystemConfiguration) {
    match TextFile::new(&system_config.logs.file).get_writer(true) {
        Ok(file) => {
            match CombinedLogger::init(vec![
                TermLogger::new(system_config.logs.level.as_level(), Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(system_config.logs.level.as_level(), Config::default(), file),
            ]) {
                Err(error) => panic!("Logging system could not be configured: {}", error),
                _ => {}
            };
        },
        Err(error) => panic!("Logging system could not be configured: {}", error)
    };
}

fn initialize_templating() -> Tera {
    match Tera::new("templates/**/*.j2") {
        Ok(tera) => tera,
        Err(error) => {
            panic!("Could not initialize HTML templates: {}", error);
        }
    }
}

fn get_cors_middleware(config: Option<Vec<String>>) -> Cors {

    let config = match config {
        Some(config) => match config.len() {
            0 => return Cors::permissive(),
            _ => config
        },
        None => return Cors::permissive()
    };

    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_header(http::header::ACCEPT)
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    for domain in config {
        cors = cors.allowed_origin(&domain);
    }

    cors

}
