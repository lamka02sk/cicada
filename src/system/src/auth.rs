use serde_json::json;
use cicada_common::CicadaResponse;
use cicada_database::{AuthLogin, CicadaHeaders, ConnectionPool, LoginForm};

pub fn login(headers: CicadaHeaders, db: &ConnectionPool, form: &mut LoginForm) -> CicadaResponse {

    let user = form.verify_credentials(&db)?;
    let token = AuthLogin::new(&db, user, headers.user_agent, headers.ip_address)?.token;

    Ok(json!({
        "token": token
    }))

}