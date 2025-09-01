//! Azure Key Vault Secrets Client.

use std::collections::HashMap;
use std::env;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use azure_core::credentials::{Secret, TokenCredential};
use azure_identity::{ClientSecretCredential, DefaultAzureCredential};
use azure_security_keyvault_secrets::SecretClient;
use runtime::ResourceBuilder;
use tracing::instrument;

const DEF_KV_ADDR: &str = "https://kv-credibil-demo.vault.azure.net";

pub struct AzKeyVault {
    attributes: HashMap<String, String>,
}

impl ResourceBuilder<SecretClient> for AzKeyVault {
    fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    #[instrument(name = "AzKeyVault::connect", skip(self))]
    async fn connect(self) -> Result<SecretClient> {
        let addr = env::var("KV_ADDR").unwrap_or_else(|_| DEF_KV_ADDR.into());

        let credential: Arc<dyn TokenCredential> = if cfg!(debug_assertions) {
            DefaultAzureCredential::new()
                .map_err(|e| anyhow!("could not create credential: {e}"))?
        } else {
            let tenant_id = env::var("AZURE_TENANT_ID")?;
            let client_id = env::var("AZURE_CLIENT_ID")?;
            let client_secret = env::var("AZURE_CLIENT_SECRET")?;
            let secret = Secret::new(client_secret);
            ClientSecretCredential::new(&tenant_id, client_id, secret, None)?
        };

        let client = SecretClient::new(&addr, credential, None)
            .map_err(|e| anyhow!("failed to connect to azure keyvault: {e}"))?;
        tracing::info!("connected to azure keyvault");

        Ok(client)
    }
}

impl IntoFuture for AzKeyVault {
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;
    type Output = Result<SecretClient>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.connect())
    }
}
