#[macro_use]
extern crate diesel_migrations;

use std::io;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use log::error;

embed_migrations!();
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
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

    pub fn get_connection(&self) -> Result<Connection, PoolError> {
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