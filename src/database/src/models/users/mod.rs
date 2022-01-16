pub mod token;
pub mod new;
pub mod change_password;
pub mod update;
pub mod security;

use chrono::NaiveDateTime;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, select, update};
use diesel::dsl::exists;
use uuid::Uuid;
use cicada_common::CicadaResult;
use cicada_common::crypto::password::verify_password;
use crate::{ConnectionPool, DbResult, get_connection, result};
use crate::schema::users;
use crate::auth::login::AuthLogin;
use crate::change_password::ChangePassword;
use crate::token::TokenUpdateUser;

const TOKEN_STRENGTH: usize = 96;

#[derive(Debug, Queryable, Serialize, Clone, Identifiable)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(skip)]
    password: String,
    #[serde(skip)]
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

    pub fn from_auth_login(db: &ConnectionPool, auth_login: &AuthLogin) -> DbResult<Self> {
        result(
            users::dsl::users
                .filter(users::dsl::id.eq(auth_login.user_id))
                .filter(users::dsl::enabled.eq(true))
                .first::<Self>(&get_connection(db)?)
        )
    }

    pub fn from_uuid(db: &ConnectionPool, uuid: Uuid) -> DbResult<Self> {
        result(
            users::dsl::users
                .filter(users::dsl::uuid.eq(uuid))
                .first::<Self>(&get_connection(db)?)
        )
    }

    pub fn update_token(&self, db: &ConnectionPool, token: TokenUpdateUser) -> DbResult<usize> {
        result(update(self).set(&token).execute(&get_connection(db)?))
    }

    pub fn update_password(&self, db: &ConnectionPool, password: ChangePassword) -> DbResult<usize> {
        result(update(self).set(&password).execute(&get_connection(db)?))
    }

}
