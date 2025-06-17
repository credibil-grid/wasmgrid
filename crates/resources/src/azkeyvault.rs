use std::env;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use azure_core::credentials::{Secret, TokenCredential};
use azure_identity::{ClientSecretCredential, DefaultAzureCredential};
use azure_security_keyvault_secrets::SecretClient;
use tokio::task::JoinHandle;

use super::{Resources, timeout};

const CONNECTION_TIMEOUT: u64 = 100; // milliseconds

impl Resources {
    pub fn with_azkeyvault(
        &self, addr: impl AsRef<str> + Send + 'static,
    ) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
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

            let client = SecretClient::new(addr.as_ref(), credential, None)
                .map_err(|e| anyhow!("failed to connect to azure keyvault: {e}"))?;
            tracing::info!("connected to azure keyvault");

            resources
                .azkeyvault
                .set(client)
                .map_err(|_| anyhow!("issue setting az keyvault client"))
        })
    }

    /// Get the Azure Keyvault client for keys.
    ///
    /// This method will block until the client is available, timing out after
    /// `CONNECTION_TIMEOUT` ms.
    ///
    /// # Errors
    ///
    /// This method returns an error if the client is not available before the
    /// method times out.
    pub fn azkeyvault(&self) -> Result<&SecretClient> {
        tracing::debug!("getting azkeyvault client");
        timeout(&self.azkeyvault, CONNECTION_TIMEOUT)
    }
}
