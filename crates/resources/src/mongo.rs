use anyhow::{Result, anyhow};
use tokio::task::JoinHandle;

use super::{Resources, timeout};

const CONNECTION_TIMEOUT: u64 = 100; // milliseconds

impl Resources {
    /// Add a MongoDB connection from a `mongodb` uri.
    ///
    /// The method will attempt connect on a separate, returning a
    /// [`tokio::task::JoinHandle`] that can be awaited if desired.
    pub fn with_mongo(&self, uri: impl AsRef<str> + Send + 'static) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let client = mongodb::Client::with_uri_str(uri).await.map_err(|e| {
                tracing::error!("failed to connect to mongo: {e}");
                anyhow!("failed to connect to mongo: {e}")
            })?;
            tracing::info!("connected to mongo");
            resources.mongo.set(client).map_err(|_| {
                tracing::error!("failed to initialize mongo context");
                anyhow!("failed to initialize mongo context")
            })
        })
    }

    /// Get the MongoDB client.
    ///
    /// This method will block until the client is available, timing out after
    /// `CONNECTION_TIMEOUT` ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    pub fn mongo(&self) -> Result<&mongodb::Client> {
        tracing::debug!("getting mongodb client");
        timeout(&self.mongo, CONNECTION_TIMEOUT)
    }
}
