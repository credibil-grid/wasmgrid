use anyhow::{Result, anyhow};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_keys::KeyClient;
use tokio::task::JoinHandle;

use crate::resources::{Resources, timeout};

impl Resources {
    pub fn with_azkeyvault(
        &self, addr: impl AsRef<str> + Send + 'static,
    ) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let credential = if cfg!(debug_assertions) {
                DefaultAzureCredential::new()
                    .map_err(|e| anyhow!("could not create credential: {e}"))?
            } else {
                // let credential = ClientSecretCredential::new()?;
                DefaultAzureCredential::new()
                    .map_err(|e| anyhow!("could not create credential: {e}"))?
            };

            let client = KeyClient::new(addr.as_ref(), credential, None).map_err(|e| {
                tracing::error!("failed to connect to azure keyvault: {e}");
                anyhow!("failed to connect to azure keyvault: {e}")
            })?;
            tracing::info!("connected to azure keyvault");
            resources.azkeyvault.set(client).map_err(|_| {
                tracing::error!("failed to initialize mongo context");
                anyhow!("failed to set az keyvault client")
            })
        })
    }

    /// Get the Azure Keyvault client.
    ///
    /// This method will block until the client is available, timing out after
    /// 100ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    pub(crate) fn azkeyvault(&self) -> Result<&KeyClient> {
        tracing::debug!("getting azkeyvault client");
        timeout(&self.azkeyvault)
    }
}
