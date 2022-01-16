use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, update};
use cicada_common::CicadaResult;
use crate::{ConnectionPool, DbResult, get_connection, result, User};
use crate::schema::user_security;

#[derive(Debug, Queryable, Serialize)]
pub struct UserSecurity {
    #[serde(skip)]
    id: i32,
    #[serde(skip)]
    user_id: i32,
    pub login_duration: i32,
    pub two_factor: bool,
    #[serde(skip)]
    pub created_at: NaiveDateTime,
    #[serde(skip)]
    updated_at: NaiveDateTime
}

impl UserSecurity {

    pub fn from_user(db: &ConnectionPool, user: &User) -> CicadaResult<Self> {
        result(
            user_security::dsl::user_security
                .filter(user_security::dsl::user_id.eq(user.id))
                .get_result(&get_connection(db)?)
        )
    }

    pub fn get_login_duration_in_seconds(&self) -> i32 {
        self.login_duration * 86400
    }

}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "user_security"]
pub struct NewUserSecurity {
    pub user_id: i32
}

impl NewUserSecurity {

    pub fn create(db: &ConnectionPool, user_id: i32) -> CicadaResult<usize> {
        result(
            insert_into(user_security::dsl::user_security)
                .values(&Self {
                    user_id
                })
                .execute(&get_connection(db)?)
        )
    }

}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "user_security"]
pub struct UpdateUserSecurity {
    pub login_duration: i32,
    pub two_factor: bool
}

impl UpdateUserSecurity {

    pub fn update(&self, db: &ConnectionPool, user: &User) -> DbResult<usize> {
        let user_security = user_security::dsl::user_security.filter(user_security::dsl::user_id.eq(user.id));
        result(update(user_security).set(self).execute(&get_connection(db)?))
    }

}
