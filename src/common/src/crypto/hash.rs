use ring::hmac::{HMAC_SHA512, Key, sign};
use crate::CicadaResult;

pub fn hmac_sign(key: &str, message: &str) -> CicadaResult<String> {
    let key = Key::new(HMAC_SHA512, key.as_bytes());
    Ok(base64::encode(sign(&key, message.as_bytes()).as_ref()))
}