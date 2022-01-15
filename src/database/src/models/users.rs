use chrono::NaiveDateTime;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, select, insert_into, update};
use diesel::dsl::exists;
use uuid::Uuid;
use cicada_common::CicadaResult;
use cicada_common::crypto::password::{hash_password, verify_password};
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, DbResult, get_connection, result};
use crate::schema::users;
use crate::auth::login::AuthLogin;
use crate::user_security::NewUserSecurity;

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

}

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

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct TokenUpdateUser {
    token: String
}

impl TokenUpdateUser {
    pub fn new() -> CicadaResult<Self> {
        Ok(Self {
            token: token(TOKEN_STRENGTH)?
        })
    }
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

        self.token = token(TOKEN_STRENGTH)?;
        self.password = hash_password(&self.password)?;

        self.admin = admin;
        self.enabled = true;

        let user_id = result(
            insert_into(users::dsl::users).values(&*self).returning(users::dsl::id).get_result::<i32>(&conn)
        )?;

        NewUserSecurity::create(db, user_id)?;
        Ok(String::new())

    }

}
