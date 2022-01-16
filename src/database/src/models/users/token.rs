use cicada_common::CicadaResult;
use cicada_common::crypto::random::token;
use crate::models::users::TOKEN_STRENGTH;
use crate::schema::users;

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct TokenUpdateUser {
    token: String
}

impl TokenUpdateUser {
    pub fn new() -> CicadaResult<Self> {
        Ok(Self {
            token: token(TOKEN_STRENGTH)?
        })
    }
}