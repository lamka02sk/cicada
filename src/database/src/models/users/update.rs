use diesel::{RunQueryDsl, update};
use uuid::Uuid;
use crate::{ConnectionPool, DbResult, get_connection, result};
use crate::schema::users;

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct SelfUpdateUser {
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String
}

impl SelfUpdateUser {
    pub fn update(&self, db: &ConnectionPool) -> DbResult<usize> {
        result(update(users::table).set(self).execute(&get_connection(db)?))
    }
}