use std::net::IpAddr;
use cicada_common::{AppError, CicadaResult};
use crate::{ConnectionPool, User};
use crate::auth::attempts::AuthAttempt;

const LOGIN_COOLDOWN_MINUTES: i64 = 5;
const LOGIN_COOLDOWN_ATTEMPTS: i64 = 3;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    password: String
}

impl LoginForm {

    pub fn verify_credentials(&self, db: &ConnectionPool, user_agent: &str, ip_address: IpAddr) -> CicadaResult<User> {

        let user = User::from_email(db, &self.email)?;

        if AuthAttempt::count(db, &user, LOGIN_COOLDOWN_MINUTES)? > LOGIN_COOLDOWN_ATTEMPTS {
            return AppError::new("auth_attempts_limit", &format!("The limit of login attempts was exceeded for user ({})", user.uuid));
        }

        if let Err(error) = user.verify_password(&self.password) {
            AuthAttempt::new(db, &user, user_agent, ip_address)?;
            return Err(error);
        }

        Ok(user)

    }

}