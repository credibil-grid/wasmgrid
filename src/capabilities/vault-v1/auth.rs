use anyhow::bail;
use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthType, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use reqwest::Url;
use serde::{Deserialize, Serialize};

const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";
const AUDIENCE: &str = "https://vault.azure.net";

/// Access token.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessToken {
    pub token: String,
    pub expires: DateTime<Utc>,
}

/// Access token implementation.
impl AccessToken {
    /// Access token as a string.
    pub fn as_str(&self) -> &str {
        self.token.as_str()
    }

    /// Get the access token using environment variables. The following environment variables are
    /// required:
    ///
    /// | Variable              | Description                                                      |
    /// |-----------------------|------------------------------------------------------------------|
    /// | `AZURE_TENANT_ID`     | The Azure Active Directory tenant(directory) ID.                 |
    /// | `AZURE_CLIENT_ID`     | The client(application) ID of an App Registration in the tenant. |
    /// | `AZURE_CLIENT_SECRET` | A client secret that was generated for the App Registration.     |
    pub async fn get_token() -> anyhow::Result<Self> {
        let Ok(tenant) = std::env::var("AZURE_TENANT_ID") else {
            bail!("AZURE_TENANT_ID environment variable not set")
        };
        let Ok(client) = std::env::var("AZURE_CLIENT_ID") else {
            bail!("AZURE_CLIENT_ID environment variable not set")
        };
        let Ok(secret) = std::env::var("AZURE_CLIENT_SECRET") else {
            bail!("AZURE_CLIENT_SECRET environment variable not set")
        };

        let t_url = Url::parse(&format!("{AZURE_PUBLIC_CLOUD}/{tenant}/oauth2/v2.0/token"))?;
        let token_url = TokenUrl::from_url(t_url);
        let a_url = Url::parse(&format!("{AZURE_PUBLIC_CLOUD}/{tenant}/oauth2/v2.0/authorize"))?;
        let auth_url = AuthUrl::from_url(a_url);

        let client = BasicClient::new(
            ClientId::new(client),
            Some(ClientSecret::new(secret)),
            auth_url,
            Some(token_url),
        )
        .set_auth_type(AuthType::RequestBody);
        let Ok(token_res) = client
            .exchange_client_credentials()
            .add_scope(Scope::new(format!("{AUDIENCE}/.default")))
            .request_async(async_http_client)
            .await
        else {
            bail!("Failed to get access token.")
        };

        Ok(Self {
            token: token_res.access_token().secret().to_string(),
            expires: Utc::now() + token_res.expires_in().unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Get an access token without error.
    #[tokio::test]
    #[ignore]
    async fn get_token() {
        let token = AccessToken::get_token().await;
        assert!(token.is_ok());
        println!("{}", token.expect("failed to retrieve access token").as_str());
    }
}
