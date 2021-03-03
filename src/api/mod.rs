use hmac::{Hmac, Mac, NewMac};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient, Method, Response as HttpResponse,
};
use sha2::{Digest, Sha256, Sha512};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

mod errors;
use errors::ApiError;

#[derive(Serialize)]
pub struct FormData {
    data: HashMap<String, String>,
}

// API-Sign = Message signature using HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key
// See : https://www.kraken.com/features/api#general-usage
pub struct Api {
    base_url: String,
    key: String,
    secret: String,
    totp: Option<String>,
    http_client: HttpClient,
}

impl Api {
    pub fn new(base_url: String, key: String, secret: String, totp: Option<String>) -> Self {
        Api {
            base_url,
            key,
            secret,
            totp,
            // A reusable reqwest http client. This client implements
            // async/await in a thread safe way (it uses [`Arc`] internally).
            // It also holds a connection pool internally.
            // See : https://docs.rs/reqwest/0.11.1/reqwest/struct.Client.html
            http_client: HttpClient::new(),
        }
    }

    pub async fn public_call(&self, path: &str) -> Result<HttpResponse, ApiError> {
        let url = build_url(&self.base_url, path)?.to_string();
        let response = self.http_client.request(Method::GET, &url).send().await?;

        Ok(response)
    }

    pub async fn private_call(
        &self,
        path: &str,
        mut data: HashMap<String, String>,
    ) -> Result<HttpResponse, ApiError> {
        let url = build_url(&self.base_url, path)?.to_string();
        let nonce = self.nonce()?.to_string();

        data.insert("nonce".to_string(), nonce.to_string());
        if let Some(ref otp) = self.totp {
            data.insert("otp".to_string(), otp.to_string());
        }

        let postdata = self.url_encode_hashmap(&data);
        let input = self.inner_sign(path, &postdata, nonce.to_string())?;
        let api_sign = self.sign(input)?;

        let mut headers = HeaderMap::new();
        headers.insert("API-Key", self.key.parse::<HeaderValue>()?);
        headers.insert("API-Sign", api_sign.parse::<HeaderValue>()?);

        let response = self
            .http_client
            .request(Method::POST, &url)
            .headers(headers)
            .body(postdata)
            .send()
            .await?;

        Ok(response)
    }

    /// Generate an increasing nonce number.
    fn nonce(&self) -> Result<u64, ApiError> {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        Ok(nonce)
    }

    fn sign(&self, input: Vec<u8>) -> Result<String, ApiError> {
        type HmacSha512 = Hmac<Sha512>;

        let key = base64::decode(self.secret.to_string())?;
        let mut mac = HmacSha512::new_varkey(&key).unwrap();
        mac.update(&input);

        let result = mac.finalize();
        let code: Vec<u8> = result.into_bytes().to_vec();
        let res = base64::encode(&code);

        Ok(res)
    }

    fn inner_sign(&self, path: &str, postdata: &str, nonce: String) -> Result<Vec<u8>, ApiError> {
        let input = [nonce, postdata.to_string()].concat();
        let bytes = input.as_bytes();

        let hashed: [u8; 32] = Sha256::digest(bytes).into();
        let res = [path.as_bytes(), &hashed].concat();

        Ok(res)
    }

    // See : https://github.com/hugues31/coinnect/blob/master/src/helpers/mod.rs#L14
    fn url_encode_hashmap(&self, hashmap: &HashMap<String, String>) -> String {
        if hashmap.is_empty() {
            return "".to_string();
        }
        let mut acc = "".to_string();

        for (k, v) in hashmap {
            acc += &(k.to_string() + "=" + v + "&");
        }

        acc.pop(); // remove the last "&"
        acc
    }
}

fn build_url(base: &str, path: &str) -> Result<Url, url::ParseError> {
    let endpoint_url = format!("{}{}", base, path);

    Url::parse(&endpoint_url)
}

#[cfg(test)]
mod tests {
    use super::Api;
    use crate::config::Config;
    use std::{collections::HashMap, time::Duration};

    fn api_setup() -> Api {
        let config = Config::new();

        Api::new(
            config.api_base_url,
            config.api_key,
            config.api_secret,
            config.totp_pwd,
        )
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

    #[tokio::test]
    async fn it_should_retrieve_open_orders() {
        let res = api_setup()
            .private_call("/0/private/OpenOrders", HashMap::new())
            .await
            .unwrap()
            .text()
            .await;

        assert!(res.is_ok());

        let orders = res.unwrap();
        println!("Orders : {}", orders);
    }

    #[tokio::test]
    async fn it_should_retrieve_open_orders_with_userref() {
        let mut postdata = HashMap::<String, String>::new();
        postdata.insert("userref".to_string(), "0".to_string());

        let res = api_setup()
            .private_call("/0/private/OpenOrders", postdata)
            .await
            .unwrap()
            .text()
            .await;

        assert!(res.is_ok());

        let orders = res.unwrap();
        println!("Orders : {}", orders);
    }

    #[tokio::test]
    async fn it_should_retrieve_server_time() {
        let res = api_setup()
            .public_call("/0/public/Time")
            .await
            .unwrap()
            .text()
            .await;

        assert!(res.is_ok());

        let time = res.unwrap();
        println!("Time : {}", time);
    }
}
