mod routes;
mod extractors;
mod middleware;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;
extern crate simplelog;

use actix_cors::Cors;
use actix_web::{App, http, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::middleware::normalize::TrailingSlash;
use actix_web::web::Data;
use cicada_common::{Cicada, DatabaseConfiguration, FileManager, SystemConfiguration, TextFile};
use tera::Tera;
use simplelog::*;
use cicada_database::{ConnectionPool, run_migrations};
use crate::middleware::auth::AuthenticateMiddlewareFactory;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    // Configuration
    let configuration = Cicada::new();
    let configuration = configuration.config.lock();

    if let Err(error) = configuration {
        panic!("Could not lock the application data before server setup: {}", error);
    }

    let configuration = configuration.unwrap();
    let system_config: &SystemConfiguration = configuration.get("system").unwrap().as_any().downcast_ref().unwrap();
    let database_config: &DatabaseConfiguration = configuration.get("database").unwrap().as_any().downcast_ref().unwrap();

    // Logging
    initialize_logging(&system_config);

    // Tera templates
    info!("Initializing HTML templating engine...");
    let tera = initialize_templating();

    // Database connection pool
    info!("Preparing database connection pool...");
    let pool  = ConnectionPool::new(&database_config.get_database_url());

    // Run database migrations
    info!("Running database migrations...");
    run_migrations(&pool);

    // Server startup
    info!("Preparing actix-web server...");
    let cors = system_config.cors.clone();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(AuthenticateMiddlewareFactory::new())
            .wrap(get_cors_middleware(cors.clone()))
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D"))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(Data::new(Cicada::new()))
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(pool.clone()))
            .configure(routes::configure)
            .default_service(routes::default())
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

    info!("{} is starting...", system_config.name);

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
