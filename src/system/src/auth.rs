use serde_json::json;
use cicada_common::CicadaResponse;
use cicada_database::{CicadaHeaders, ConnectionPool};
use cicada_database::auth::login_form::LoginForm;
use cicada_database::auth::login::AuthLogin;

pub fn login(headers: CicadaHeaders, db: &ConnectionPool, form: &mut LoginForm) -> CicadaResponse {

    let user = form.verify_credentials(&db, &headers.user_agent, headers.ip_address)?;
    let token = AuthLogin::new(&db, &user, &headers.user_agent, headers.ip_address)?;

    Ok(json!({
        "token": token
    }))

}

pub fn logout(db: &ConnectionPool, login: &AuthLogin) -> CicadaResponse {
    login.deactivate(&db)?;
    Ok(json!({}))
}