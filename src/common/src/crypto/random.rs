use crate::{AppError, CicadaResult};

pub fn token(strength: usize) -> CicadaResult<String> {

    let mut random_bytes = vec![0u8; strength];

    if let Err(error) = openssl::rand::rand_bytes(&mut random_bytes) {
        return AppError::new::<String>("create_user_token_rand", &error.to_string());
    }

    Ok(base64::encode(&random_bytes))

}