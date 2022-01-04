use crate::{CicadaError, CicadaErrorLog, CicadaResult};

pub fn token(strength: usize) -> CicadaResult<String> {

    let mut random_bytes = vec![0u8; strength];

    if let Err(error) = openssl::rand::rand_bytes(&mut random_bytes) {
        return CicadaError::log_real(CicadaErrorLog::Error, error.into());
    }

    Ok(base64::encode(&random_bytes))

}