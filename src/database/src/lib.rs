mod models;
mod schema;

use std::io;
pub use models::*;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;
extern crate uuid;

use diesel::{PgConnection, QueryResult};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use log::error;

embed_migrations!();
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub type ConnectionResult = Result<Connection, PoolError>;
pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct ConnectionPool(DbPool);

impl ConnectionPool {

    pub fn new(database_url: &str) -> Self {

        let manager: ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
        let pool: Result<DbPool, PoolError> = Pool::builder().build(manager);

        if let Err(error) = pool {
            error!("Database connection pool failed: {}", error);
            panic!("Database connection pool failed: {}", error);
        }

        Self(pool.unwrap())

    }

    pub fn get_connection(&self) -> ConnectionResult {
        self.0.get()
    }

}

pub fn run_migrations(pool: &ConnectionPool) {

    let connection = match pool.get_connection() {
        Ok(connection) => connection,
        Err(error) => {
            error!("Could not connect to the database to run migrations: {}", error);
            panic!("Could not connect to the database to run migrations: {}", error);
        }
    };

    if let Err(error) = embedded_migrations::run_with_output(&connection, &mut io::stdout()) {
        error!("Database migrations failed: {}", error);
        panic!("Database migrations failed: {}", error);
    }

}

pub type DbResult<T> = Result<T, String>;

pub fn result<T>(result: QueryResult<T>) -> DbResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(error.to_string())
    }
}