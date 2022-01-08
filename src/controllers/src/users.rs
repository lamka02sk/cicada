use serde_json::json;
use cicada_common::{CicadaError, CicadaHttpLog, CicadaResponse};
use cicada_database::{ConnectionPool, SelfUpdateUser, User};
use cicada_database::auth::login::{AuthLogin, UUIDAuthLogin};

pub fn update_self(db: &ConnectionPool, user: &SelfUpdateUser) -> CicadaResponse {
    user.update(db)?;
    Ok(json!({}))
}

pub fn get_logins(db: &ConnectionPool, user: &User) -> CicadaResponse {
    Ok(json!({
        "logins": AuthLogin::from_user(db, user)?
    }))
}

pub fn disable_login(db: &ConnectionPool, user: &User, login: &UUIDAuthLogin) -> CicadaResponse {

    let auth_login = AuthLogin::from_uuid(db, &login.uuid)?;
    let auth_login_user = User::from_auth_login(db, &auth_login)?;

    auth_login.deactivate(db)?;

    match auth_login_user.id == user.id {
        true => Ok(json!({})),
        false => CicadaError::forbidden(CicadaHttpLog::Default)
    }

}
