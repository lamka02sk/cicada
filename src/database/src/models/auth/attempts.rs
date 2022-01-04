use std::net::IpAddr;
use chrono::{Duration, Local, NaiveDateTime};
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl};
use ipnetwork::IpNetwork;
use uuid::Uuid;
use cicada_common::CicadaResult;
use crate::{ConnectionPool, get_connection, result, result_any, User};
use crate::schema::auth_attempts;

#[derive(Queryable, Serialize)]
pub struct AuthAttempt {
    id: i32,
    pub uuid: Uuid,
    pub user_id: i32,
    pub user_agent: String,
    pub ip_address: IpNetwork,
    pub created_at: NaiveDateTime
}

impl AuthAttempt {

    pub fn new(db: &ConnectionPool, user: &User, user_agent: &str, ip_address: IpAddr) -> CicadaResult<NewAuthAttempt> {

        let attempt = NewAuthAttempt {
            user_id: user.id,
            user_agent: user_agent.to_string(),
            ip_address: ip_address.into()
        };

        result_any(insert_into(auth_attempts::dsl::auth_attempts).values(&attempt).execute(&get_connection(db)?))?;
        Ok(attempt)

    }

    pub fn count(db: &ConnectionPool, user: &User, timeframe: i64) -> CicadaResult<i64> {

        let datetime = Local::now() - Duration::minutes(timeframe);

        result(
            auth_attempts::dsl::auth_attempts.count()
                .filter(auth_attempts::dsl::user_id.eq(user.id))
                .filter(auth_attempts::dsl::created_at.ge(datetime.naive_local()))
                .get_result::<i64>(&get_connection(db)?)
        )

    }

}

#[derive(Insertable)]
#[table_name = "auth_attempts"]
pub struct NewAuthAttempt {
    pub user_id: i32,
    pub user_agent: String,
    pub ip_address: IpNetwork
}