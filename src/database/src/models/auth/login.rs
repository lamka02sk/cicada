use std::net::IpAddr;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl};
use ipnetwork::IpNetwork;
use uuid::Uuid;
use cicada_common::CicadaResult;
use cicada_common::crypto::hash::hmac_sign;
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, get_connection, result_any, User};
use crate::schema::auth_login;

const TOKEN_STRENGTH: usize = 96;

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct AuthLogin {
    id: i32,
    pub uuid: Uuid,
    pub user_id: i32,
    secret: String,
    token: String,
    pub user_agent: String,
    pub ip_address: IpNetwork,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl AuthLogin {

    pub fn new(db: &ConnectionPool, user: &User, user_agent: &str, ip_address: IpAddr) -> CicadaResult<NewAuthLogin> {

        let secret = token(TOKEN_STRENGTH)?;
        let token = hmac_sign(&secret, &user.token)?;

        let login = NewAuthLogin {
            user_id: user.id,
            secret,
            token,
            user_agent: user_agent.to_string(),
            ip_address: ip_address.into(),
            active: true
        };

        result_any(insert_into(auth_login::dsl::auth_login).values(&login).execute(&get_connection(db)?))?;
        Ok(login)

    }

    pub fn from_token(db: &ConnectionPool, token: &str) -> Option<Self> {

        let conn = match get_connection(&db) {
            Ok(conn) => conn,
            _ => return None
        };

        match auth_login::dsl::auth_login
            .filter(auth_login::dsl::token.eq(token))
            .filter(auth_login::dsl::active.eq(true))
            .first::<Self>(&conn) {
            Ok(login) => Some(login),
            _ => None
        }

    }

}

#[derive(Insertable)]
#[table_name = "auth_login"]
pub struct NewAuthLogin {
    pub user_id: i32,
    secret: String,
    pub token: String,
    pub user_agent: String,
    pub ip_address: IpNetwork,
    pub active: bool
}