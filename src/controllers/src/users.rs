use serde_json::json;
use cicada_common::{CicadaError, CicadaHttpLog, CicadaResponse, CicadaResult};
use cicada_database::{ConnectionPool, User};
use cicada_database::auth::login::{AuthLogin, UUIDAuthLogin};
use cicada_database::change_password::ChangePasswordForm;
use cicada_database::token::TokenUpdateUser;
use cicada_database::update::SelfUpdateUser;
use cicada_database::security::{UpdateUserSecurity, UserSecurity};

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

pub fn get_security(db: &ConnectionPool, user: &User) -> CicadaResponse {
    Ok(json!({
        "user_security": UserSecurity::from_user(db, user)?
    }))
}

pub fn update_security(db: &ConnectionPool, user: &User, security: &UpdateUserSecurity) -> CicadaResponse {
    security.update(db, user)?;
    Ok(json!({}))
}

pub fn token_refresh(db: &ConnectionPool, user: &User) -> CicadaResponse {
    let token = TokenUpdateUser::new()?;
    user.update_token(db, token)?;
    Ok(json!({}))
}

pub fn change_password(db: &ConnectionPool, user: &User, passwords: &ChangePasswordForm) -> CicadaResponse {

    if !user.verify_password(&passwords.old_password)? {
        return CicadaError::make_public(CicadaError::forbidden(CicadaHttpLog::Custom("Old password could not be verified".to_string())));
    }

    user.update_password(db, CicadaResult::from(passwords)?)?;
    Ok(json!({}))

}
