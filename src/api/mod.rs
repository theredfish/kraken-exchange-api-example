use hmac::{Hmac, Mac, NewMac};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

mod errors;

use errors::ApiError;

// API-Sign = Message signature using HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key
// See : https://www.kraken.com/features/api#general-usage
pub struct Api {
    key: String,
    secret: String,
    base_url: String,
}

impl Api {
    pub fn new(key: String, secret: String, base_url: String) -> Self {
        Api {
            key,
            secret,
            base_url,
        }
    }

    /// Generate an increasing nonce number.
    pub fn nonce(&self) -> Result<u64, ApiError> {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        Ok(nonce)
    }

    pub fn sign(key: &str, input: Vec<u8>) -> Result<String, ApiError> {
        type HmacSha512 = Hmac<Sha512>;

        // Based on https://git.cryptid.cc/lost/kraken_api/src/branch/master/src/crypto.rs
        let key = base64::decode(key)?;
        let mut mac = HmacSha512::new_varkey(&key).unwrap();
        mac.update(&input);

        let result = mac.finalize();
        let code: Vec<u8> = result.into_bytes().to_vec();
        let res = base64::encode(&code);

        Ok(res)
    }

    pub fn inner_sign(&self, path: &str, data: String) -> Result<Vec<u8>, ApiError> {
        // Based on https://git.cryptid.cc/lost/kraken_api/src/branch/master/src/crypto.rs
        let nonce = self.nonce()?.to_string();
        let input = [nonce, data].concat();
        let bytes = input.as_bytes();

        let hashed: [u8; 32] = Sha256::digest(bytes).into();
        let res = [path.as_bytes(), &hashed].concat();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Api;

    fn api_setup() -> Api {
        let key = String::from("key");
        let secret = String::from("secret");
        let base_url = String::from("https://api.base.com");

        Api::new(key, secret, base_url)
    }

    #[test]
    fn second_nonce_should_be_greater_than_first_one() {
        use std::thread::sleep;

        let api = api_setup();
        let nonce1 = api.nonce().expect("Should be a valid nonce");
        sleep(Duration::from_millis(1));
        let nonce2 = api.nonce().expect("Should be a valid nonce");

        assert!(
            nonce2 > nonce1,
            "nonce 2({}) should be greater than nonce1({})",
            nonce2,
            nonce1
        );
    }
}
