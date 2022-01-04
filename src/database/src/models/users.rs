use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, QueryDsl, ExpressionMethods, RunQueryDsl, select, insert_into};
use diesel::dsl::exists;
use uuid::Uuid;
use cicada_common::CicadaResult;
use cicada_common::crypto::password::{hash_password, verify_password};
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, DbResult, get_connection, result, result_any};
use crate::schema::users;

const TOKEN_STRENGTH: usize = 96;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    password: String,
    pub token: String,
    pub admin: bool,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl User {

    pub fn verify_password(&self, password: &str) -> CicadaResult<bool> {
        verify_password(password, &self.password)
    }

    pub fn exists_admin(db: &ConnectionPool) -> DbResult<bool> {
        let conn = get_connection(db)?;
        result(select(exists(users::dsl::users.filter(users::dsl::admin.eq(true)))).get_result::<bool>(&conn))
    }

    pub fn from_email(db: &ConnectionPool, email: &str) -> DbResult<Self> {
        result(
            users::dsl::users
                .filter(users::dsl::email.eq(email))
                .filter(users::dsl::enabled.eq(true))
                .first::<Self>(&get_connection(db)?)
        )
    }

}

// #[derive(Debug, Identifiable, Deserialize)]
// #[table_name = "users"]
// pub struct FormUser<'a> {
//     id: i32,
//     pub uuid: Uuid,
//     pub firstname: &'a str,
//     pub lastname: &'a str,
//     pub email: &'a str,
//     password: String,
//     token: String,
//     pub admin: bool,
//     pub enabled: bool
// }

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

        self.token = token(TOKEN_STRENGTH)?;
        self.password = hash_password(&self.password)?;

        self.admin = admin;
        self.enabled = true;

        result_any(insert_into(users::dsl::users).values(&*self).execute(&conn))

    }

}
