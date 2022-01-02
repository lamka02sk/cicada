use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, QueryDsl, ExpressionMethods, RunQueryDsl, select, insert_into};
use diesel::dsl::exists;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use cicada_common::CicadaResult;
use cicada_common::crypto::password::hash_password;
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, DbResult, get_connection, result, result_any};
use crate::schema::users;

const TOKEN_STRENGTH: usize = 96;

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

impl User {

    pub fn exists_admin(db: &ConnectionPool) -> DbResult<bool> {
        let conn = get_connection(db)?;
        result(select(exists(users::dsl::users.filter(users::dsl::admin.eq(true)))).get_result::<bool>(&conn))
    }

}

#[derive(Debug, Identifiable, Deserialize)]
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

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    password: String,
    token: String,
    admin: bool,
    enabled: bool
}

impl NewUser {

    pub fn create(&mut self, db: &ConnectionPool, admin: bool) -> CicadaResult<String> {

        let conn = get_connection(db)?;

        self.token = self.generate_token()?;
        self.password = self.hash_password()?;

        self.admin = admin;
        self.enabled = true;

        result_any(insert_into(users::dsl::users).values(&*self).execute(&conn))

    }

    fn generate_token(&self) -> CicadaResult<String> {
        token(TOKEN_STRENGTH)
    }

    fn hash_password(&mut self) -> CicadaResult<String> {
        hash_password(&self.password)
    }

}
