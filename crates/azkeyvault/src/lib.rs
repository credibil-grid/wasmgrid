use std::env;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use azure_core::credentials::{Secret, TokenCredential};
use azure_identity::{ClientSecretCredential, DefaultAzureCredential};
use azure_security_keyvault_secrets::SecretClient;
use runtime::Resource;

pub struct AzKeyVault {
    secret_client: Arc<SecretClient>,
}

impl Resource for AzKeyVault {
    type Resource = SecretClient;

    fn identifier(&self) -> &'static str {
        "azkeyvault"
    }

    fn resource(&self) -> &Self::Resource {
        &self.secret_client
    }
}

impl AzKeyVault {
    /// Create a new Azure Key Vault secrets client.
    ///
    /// # Errors
    ///
    /// Returns an error if the client could not be created.
    pub fn new(addr: &'static str) -> Result<Self> {
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

        let client = SecretClient::new(addr, credential, None)
            .map_err(|e| anyhow!("failed to connect to azure keyvault: {e}"))?;
        tracing::info!("connected to azure keyvault");

        Ok(Self {
            secret_client: Arc::new(client),
        })
    }
}

// fn timeout<T>(once_lock: &OnceLock<T>, limit: u64) -> Result<&T> {
//     let duration = Duration::from_millis(limit / 10);
//     for _ in 0..10 {
//         if let Some(client) = once_lock.get() {
//             return Ok(client);
//         }
//         sleep(duration);
//     }
//     tracing::error!("failed to get resource");
//     Err(anyhow!("failed to get resource"))
// }
