use serde_json::json;
use cicada_common::{CicadaError, CicadaResponse};
use cicada_database::{ConnectionPool, User};

pub fn get_status(db: &ConnectionPool) -> CicadaResponse {

    if let Err(error) = db.get_connection() {
        return CicadaError::internal(&error.to_string());
    }

    Ok(json!({
        "ready": match User::exists_admin(&db.get_connection().unwrap()) {
            Ok(ready) => ready,
            Err(error) => return CicadaError::internal(&error)
        }
    }))

}
