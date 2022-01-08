use serde_json::json;
use cicada_common::CicadaResponse;
use cicada_database::{ConnectionPool, SelfUpdateUser, User};
use cicada_database::auth::login::AuthLogin;

pub fn update_self(db: &ConnectionPool, user: &SelfUpdateUser) -> CicadaResponse {
    user.update(db)?;
    Ok(json!({}))
}

pub fn get_logins(db: &ConnectionPool, user: &User) -> CicadaResponse {
    Ok(json!({
        "logins": AuthLogin::from_user(db, user)?
    }))
}