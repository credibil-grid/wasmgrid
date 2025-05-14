use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_nats::{AuthError, ConnectOptions};
use tokio::task::JoinHandle;

use crate::resources::{Resources, timeout};

impl Resources {
    /// Add a NATS connection using the given address and, optionally,
    /// authenticating using a NATS `nkeys` JWT and seed.
    ///
    /// The method will attempt connect on a separate, returning a
    /// [`tokio::task::JoinHandle`] that can be awaited if desired.
    pub fn with_nats(
        &self, addr: impl Into<String> + Send + 'static, jwt: Option<String>, seed: Option<String>,
    ) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let client = nats_connect(addr.into(), jwt, seed).await.map_err(|e| {
                tracing::error!("failed to connect to nats: {e}");
                anyhow!("failed to connect to nats: {e}")
            })?;
            tracing::info!("connected to nats");
            resources.nats.set(client).map_err(|_| {
                tracing::error!("failed to initialize nats context");
                anyhow!("failed to initialize nats context")
            })
        })
    }

    /// Get the NATS client.
    ///
    /// This method will block until the client is available, timing out after
    /// 100ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    pub(crate) fn nats(&self) -> Result<&async_nats::Client> {
        timeout(&self.nats)
    }
}

async fn nats_connect(
    addr: String, jwt: Option<String>, seed: Option<String>,
) -> Result<async_nats::Client> {
    let mut opts = ConnectOptions::new();
    if let Some(jwt) = jwt {
        let key_pair = nkeys::KeyPair::from_seed(&seed.unwrap_or_default())
            .map_err(|e| anyhow!("failed to create KeyPair: {e}"))?;
        let key_pair = Arc::new(key_pair);
        opts = opts.jwt(jwt, move |nonce| {
            let key_pair = key_pair.clone();
            async move { key_pair.sign(&nonce).map_err(AuthError::new) }
        });
    }
    opts.connect(addr).await.map_err(|e| anyhow!("{e}"))
}
