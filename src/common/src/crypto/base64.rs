use crate::{CicadaError, CicadaResult};

pub fn encode(data: &str) -> String {
    base64::encode(data)
}

pub fn decode(data: &str) -> CicadaResult<String> {
    match base64::decode(data) {
        Ok(decoded) => Ok(String::from_utf8_lossy(&decoded[..]).to_string()),
        Err(error) => CicadaError::real(error.into())
    }
}