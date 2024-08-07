use std::env;
use std::sync::Arc;

use anyhow::{anyhow, bail};
use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthType, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use super::key::{KeyBundle, SigningRequest, SigningResponse};

const AZURE_CLOUD: &str = "https://login.microsoftonline.com";
const AUDIENCE: &str = "https://vault.azure.net";
const API_VERSION: &str = "7.4";
// const MAX_KEY_VERSIONS: usize = 25;

pub struct KeyClient {
    vault_url: String,
    http_client: reqwest::Client,
    access_token: AccessToken,
}

impl KeyClient {
    pub fn new(vault_url: impl Into<String>) -> anyhow::Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let http_client = reqwest::Client::builder().default_headers(headers).build()?;

        Ok(Self { vault_url: vault_url.into(), http_client, access_token: AccessToken::new() })
    }

    pub async fn get_key(&self, key_name: &str) -> anyhow::Result<KeyBundle> {
        let token = self.access_token.token().await?;
        let key_url = format!("{}/keys/{key_name}?api-version={API_VERSION}", self.vault_url);

        let resp = self.http_client.get(&key_url).bearer_auth(&token).send().await?;
        if resp.status().is_success() {
            resp.json::<KeyBundle>()
                .await
                .map_err(|e| anyhow!("unable to deserialize response: {e}"))
        } else {
            let az_err =
                resp.json::<AzError>().await.map_err(|e| anyhow!("issue getting key: {e}"))?;
            bail!("code: {}, message: {}", az_err.error.code, az_err.error.message)
        }
    }

    pub async fn sign(&self, key_name: &str, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let body = SigningRequest {
            alg: "ES256K".to_string(),
            value: Base64UrlUnpadded::encode_string(&Sha256::digest(data)),
        };
        let token = self.access_token.token().await?;
        let key_url = format!("{}/keys/{key_name}/sign?api-version={API_VERSION}", self.vault_url);

        let resp = self.http_client.post(&key_url).bearer_auth(token).json(&body).send().await?;
        if resp.status().is_success() {
            let sig_resp = resp
                .json::<SigningResponse>()
                .await
                .map_err(|e| anyhow!("unable to deserialize response: {e}"))?;

            Base64UrlUnpadded::decode_vec(&sig_resp.value)
                .map_err(|e| anyhow!("unable to decode signature: {e}"))

            // Ok((sig.value.into_bytes(), sig.kid))
        } else {
            let az_err =
                resp.json::<AzError>().await.map_err(|e| anyhow!("issue getting key: {e}"))?;
            bail!("code: {}, message: {}", az_err.error.code, az_err.error.message)
        }
    }
}

/// Access token.
#[derive(Debug)]
struct AccessToken {
    cache: Arc<Mutex<Cache>>,
}

/// Access token implementation.
impl AccessToken {
    fn new() -> Self {
        Self { cache: Arc::new(Mutex::new(Cache { token: String::new(), expires_at: Utc::now() })) }
    }

    /// Get access token.
    pub async fn token(&self) -> anyhow::Result<String> {
        // if cache is not expired, return the cached token
        let mut cache = self.cache.lock().await;
        if cache.expires_at.signed_duration_since(Utc::now()).num_seconds() > 20 {
            return Ok(cache.token.clone());
        }

        // cache has expired, get a new token
        let tenant_id = env::var("AZURE_TENANT_ID")?;
        let client_id = env::var("AZURE_CLIENT_ID")?;
        let client_secret = env::var("AZURE_CLIENT_SECRET")?;

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(format!("{AZURE_CLOUD}/{tenant_id}/oauth2/v2.0/authorize"))?,
            Some(TokenUrl::new(format!("{AZURE_CLOUD}/{tenant_id}/oauth2/v2.0/token"))?),
        )
        .set_auth_type(AuthType::RequestBody);

        let resp = client
            .exchange_client_credentials()
            .add_scope(Scope::new(format!("{AUDIENCE}/.default")))
            .request_async(async_http_client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get access token: {}", e))?;

        *cache = Cache {
            token: resp.access_token().secret().to_string(),
            expires_at: Utc::now() + resp.expires_in().unwrap_or_default(),
        };

        Ok(cache.token.clone())
    }
}

#[derive(Debug)]
struct Cache {
    token: String,
    expires_at: DateTime<Utc>,
}

/// Error returned from Azure Key Vault.
#[derive(Debug, Deserialize)]
struct AzError {
    error: AzErrorDetail,
}

/// Error returned from Azure Key Vault.
#[derive(Debug, Deserialize)]
struct AzErrorDetail {
    code: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use ecdsa::signature::Verifier;
    use ecdsa::{Signature, VerifyingKey};
    use k256::Secp256k1;

    use super::*;

    #[tokio::test]
    async fn get_token() {
        dotenv::dotenv().ok();

        let access_token = AccessToken::new();
        let token_1 = access_token.token().await.expect("should get token");
        assert!(token_1.len() > 0);

        let token_2 = access_token.token().await.expect("should get token");
        assert_eq!(token_1, token_2);
    }

    #[tokio::test]
    async fn get_keypair() {
        dotenv::dotenv().ok();

        let client = KeyClient::new("https://kv-credibil-demo.vault.azure.net")
            .expect("should create client");
        let keypair =
            client.get_key("demo-credibil-io-signing-key").await.expect("should get keypair");
        println!("{:?}", keypair);
    }

    #[tokio::test]
    async fn sign() {
        dotenv::dotenv().ok();

        let data = b"hello world";

        let client = KeyClient::new("https://kv-credibil-demo.vault.azure.net")
            .expect("should create client");

        let sig_bytes =
            client.sign("demo-credibil-io-signing-key", data).await.expect("should sign");

        //  verifying key
        let kv_key = client.get_key("demo-credibil-io-signing-key").await.expect("should get key");

        let mut x = Base64UrlUnpadded::decode_vec(&kv_key.public_key().x).expect("should decode x");
        let mut y = Base64UrlUnpadded::decode_vec(&kv_key.public_key().y).expect("should decode y");

        let mut sec1 = vec![0x04]; // uncompressed format
        sec1.append(&mut x);
        sec1.append(&mut y);
        let verifying_key = VerifyingKey::<Secp256k1>::from_sec1_bytes(&sec1).unwrap();

        let signature: Signature<Secp256k1> =
            Signature::from_slice(&sig_bytes).expect("should get signature");

        match verifying_key.verify(data, &signature) {
            Ok(_) => println!("VERIFICATION PASSED"),
            Err(_) => panic!("VERIFICATION FAILED"),
        }
    }
}
