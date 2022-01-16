use diesel::{insert_into, RunQueryDsl};
use cicada_common::CicadaResult;
use cicada_common::crypto::password::hash_password;
use cicada_common::crypto::random::token;
use crate::{ConnectionPool, get_connection, result};
use crate::models::users::TOKEN_STRENGTH;
use crate::schema::users;
use crate::security::NewUserSecurity;

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