pub mod auth;
pub mod users;

use serde_json::{json, Value};
use cicada_common::{CicadaError, CicadaResponse};
use cicada_database::{ConnectionPool, NewUser, User};

pub fn get_status(db: &ConnectionPool) -> CicadaResponse {
    Ok(json!({
        "ready": User::exists_admin(&db)?
    }))
}

pub fn create_admin_account(db: &ConnectionPool, user: &mut NewUser) -> CicadaResponse {

    if User::exists_admin(&db)? {
        return CicadaError::http(403, "Administrator user already exists".into());
    }

    match user.create(&db, true) {
        Ok(_) => Ok(Value::Null),
        Err(error) => Err(error)
    }

}
