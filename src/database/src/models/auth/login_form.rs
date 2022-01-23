use std::net::IpAddr;
use cicada_common::{CicadaError, CicadaHttpLog, CicadaResult};
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
            return CicadaError::too_many_requests(format!("The limit of login attempts was exceeded for user ({})", user.uuid).into());
        }

        let verify_result = user.verify_password(&self.password);

        if let Err(error) = verify_result {
            AuthAttempt::new(db, &user, user_agent, ip_address)?;
            return Err(error);
        }

        if !verify_result.unwrap() {
            return CicadaError::make_public(CicadaError::forbidden(CicadaHttpLog::Custom("Invalid credentials".to_string())));
        }

        Ok(user)

    }

}