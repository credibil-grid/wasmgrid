use std::env;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthType, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};

const AZURE_CLOUD: &str = "https://login.microsoftonline.com";
const AUDIENCE: &str = "https://vault.azure.net";

pub struct KeyClient {
    vault_url: String,
    http_client: reqwest::Client,
    access_token: AccessToken,
}

impl KeyClient {
    pub fn new(vault_url: String) -> anyhow::Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let http_client = reqwest::Client::builder().default_headers(headers).build()?;

        Ok(Self { vault_url, http_client, access_token: AccessToken::new() })
    }
}

/// Access token.
#[derive(Debug, Clone)]
pub struct AccessToken {
    cache: Arc<Mutex<Cache>>,
}

#[derive(Debug, Clone)]
struct Cache {
    token: String,
    expires_at: DateTime<Utc>,
}

/// Access token implementation.
impl AccessToken {
    fn new() -> Self {
        Self { cache: Arc::new(Mutex::new(Cache { token: String::new(), expires_at: Utc::now() })) }
    }

    /// Get access token.
    async fn token(&self) -> anyhow::Result<String> {
        let mut cache = self.cache.lock().map_err(|e| anyhow!("issue locking cache: {}", e))?;

        if cache.expires_at.signed_duration_since(Utc::now()).num_seconds() > 20 {
            return Ok(cache.token.clone());
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    // Get an access token without error.
    #[tokio::test]
    async fn get_token() {
        dotenv::dotenv().ok();

        let access_token = AccessToken::new();
        let token_1 = access_token.token().await.expect("should get token");
        assert!(token_1.len() > 0);

        let token_2 = access_token.token().await.expect("should get token");
        assert_eq!(token_1, token_2);
    }
}
