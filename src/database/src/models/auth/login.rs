use std::net::IpAddr;
use std::str::FromStr;
use std::thread;
use chrono::{Duration, Local, NaiveDateTime};
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, update};
use ipnetwork::IpNetwork;
use uuid::Uuid;
use cicada_common::{CicadaError, CicadaResult};
use cicada_common::crypto::base64::{decode, encode};
use cicada_common::crypto::hash::hmac_sign;
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, DbResult, get_connection, result, User};
use crate::schema::auth_login;
use crate::diesel::BelongingToDsl;
use crate::user_security::UserSecurity;

const TOKEN_STRENGTH: usize = 96;

#[derive(Queryable, Serialize, Associations, Clone, Debug, Identifiable)]
#[belongs_to(User)]
#[table_name = "auth_login"]
pub struct AuthLogin {
    #[serde(skip)]
    id: i32,
    pub uuid: Uuid,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    secret: String,
    pub user_agent: String,
    pub ip_address: IpNetwork,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl AuthLogin {

    /// Token format: base64(login_uuid).base64(hmac(user_token, secret))
    /// 1. Generate secret randomly
    /// 2. Create auth_login
    /// 3. Sign user_token with secret
    /// 4. Join payload and signature with a dot
    pub fn new(db: &ConnectionPool, user: &User, user_agent: &str, ip_address: IpAddr) -> CicadaResult<String> {

        let secret = token(TOKEN_STRENGTH)?;
        let signature = hmac_sign(&secret, &user.token)?;

        let login = NewAuthLogin {
            user_id: user.id,
            secret,
            user_agent: user_agent.to_string(),
            ip_address: ip_address.into(),
            active: true
        };

        let uuid: Uuid = result(insert_into(auth_login::dsl::auth_login).values(&login).returning(auth_login::dsl::uuid).get_result(&get_connection(db)?))?;
        Ok(encode(&uuid.to_string()) + "." + &signature)

    }

    /// 1. Split payload from signature
    /// 2. Load auth_login and user from DB
    /// 3. Sign user_token with secret
    /// 4. Compare signatures
    pub fn from_token(db: &ConnectionPool, token: &str) -> CicadaResult<Self> {

        let (payload, signature) = match token.split_once(".") {
            Some(value) => value,
            None => return CicadaError::default()
        };

        let payload = decode(payload)?;
        let payload = match Uuid::from_str(&payload) {
            Ok(value) => value,
            _ => return CicadaError::default()
        };

        let auth_login = Self::from_uuid(db, &payload)?;
        let user = User::from_auth_login(db, &auth_login)?;
        let security = UserSecurity::from_user(db, &user)?;

        let valid_until = Local::now() - Duration::days(security.login_duration as i64);
        if valid_until.naive_local() > auth_login.updated_at {
            return CicadaError::default();
        }

        let real_signature = hmac_sign(&auth_login.secret, &user.token)?;

        match signature == &real_signature {
            true => {
                auth_login.update_timestamp(db);
                Ok(auth_login)
            },
            false => CicadaError::default()
        }

    }

    pub fn from_uuid(db: &ConnectionPool, uuid: &Uuid) -> DbResult<Self> {
        result(
            auth_login::dsl::auth_login
                .filter(auth_login::dsl::uuid.eq(uuid))
                .filter(auth_login::dsl::active.eq(true))
                .first(&get_connection(db)?)
        )
    }

    pub fn from_user(db: &ConnectionPool, user: &User) -> DbResult<Vec<Self>> {
        result(
            Self::belonging_to(user)
                .limit(20)
                .order_by(auth_login::dsl::active.desc())
                .order_by(auth_login::dsl::id.desc())
                .get_results(&get_connection(db)?)
        )
    }

    pub fn deactivate(&self, db: &ConnectionPool) -> DbResult<usize> {
        result(update(self).set(&ActivateAuthLogin { active: false }).execute(&get_connection(db)?))
    }

    pub fn update_timestamp(&self, db: &ConnectionPool) {

        let auth_login = self.clone();
        let conn = match get_connection(db) {
            Ok(c) => c,
            _ => return
        };

        thread::spawn(move || {
            update(&auth_login).set(auth_login::dsl::updated_at.eq(Local::now().naive_local())).execute(&conn).unwrap();
        });

    }

}

#[derive(AsChangeset)]
#[table_name = "auth_login"]
struct ActivateAuthLogin {
    active: bool
}

#[derive(Insertable)]
#[table_name = "auth_login"]
pub struct NewAuthLogin {
    pub user_id: i32,
    secret: String,
    pub user_agent: String,
    pub ip_address: IpNetwork,
    pub active: bool
}

#[derive(Deserialize)]
pub struct UUIDAuthLogin {
    pub uuid: Uuid
}