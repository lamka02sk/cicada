use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;
use argon2::Version::V0x13;
use rand_core::OsRng;
use crate::{AppError, CicadaResult};

fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn hash_password(password: &str) -> CicadaResult<String> {

    let password = password.as_bytes();
    let params = match Params::new(8192, 5, 1, None) {
        Ok(params) => params,
        Err(error) => return AppError::new("password_hash_params", &format!("Password could not be hashed: {}", error)).into()
    };

    let hasher = Argon2::new(Argon2id, V0x13, params);

    match hasher.hash_password(password, &generate_salt()) {
        Ok(hash) => Ok(hash.to_string()),
        Err(error) => AppError::new("password_hash", &format!("Password could not be hashed: {}", error)).into()
    }

}

pub fn verify_password(password: &str, hash: &str) -> CicadaResult<bool> {

    let password = password.as_bytes();
    let hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(error) => return AppError::new("password_verify", &format!("Password could not be verified: {}", error)).into()
    };

    Ok(Argon2::default().verify_password(password, &hash).is_ok())

}

#[cfg(test)]
mod test {

    use crate::crypto::password::{hash_password, verify_password};

    const PASSWORD: &str = "verysecurepassword123";

    #[test]
    fn hash() {
        let hash = hash_password(PASSWORD).unwrap();
        assert_eq!(verify_password(PASSWORD, &hash).unwrap(), true);
    }

}
