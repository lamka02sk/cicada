use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, QueryDsl, ExpressionMethods, RunQueryDsl, select};
use diesel::dsl::exists;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{Connection, DbResult, result};
use crate::schema::users;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i32,
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    password: String,
    token: String,
    pub admin: bool,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable, Identifiable, Deserialize)]
#[table_name = "users"]
pub struct FormUser<'a> {
    id: i32,
    pub uuid: Uuid,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    password: String,
    token: String,
    pub admin: bool,
    pub enabled: bool
}

impl User {

    pub fn exists_admin(conn: &Connection) -> DbResult<bool> {
        result(select(exists(users::dsl::users.filter(users::dsl::admin.eq(true)))).get_result::<bool>(conn))
    }

}