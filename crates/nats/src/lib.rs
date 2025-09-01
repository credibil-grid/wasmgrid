//! NATS Client.

use std::collections::HashMap;
use std::env;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_nats::{AuthError, Client, ConnectOptions};
use runtime::ResourceBuilder;
use tracing::instrument;

const DEF_NATS_ADDR: &str = "demo.nats.io";

pub struct Nats {
    attributes: HashMap<String, String>,
}

impl ResourceBuilder<Client> for Nats {
    fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    #[instrument(name = "Nats::connect", skip(self))]
    async fn connect(self) -> Result<Client> {
        let addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
        let jwt = env::var("NATS_JWT").ok();
        let seed = env::var("NATS_SEED").ok();

        let client = connect(addr, jwt, seed).await.map_err(|e| {
            tracing::error!("failed to connect to nats: {e}");
            anyhow!("failed to connect to nats: {e}")
        })?;
        tracing::info!("connected to nats");

        Ok(client)
    }
}

impl IntoFuture for Nats {
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;
    type Output = Result<Client>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.connect())
    }
}

async fn connect(
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
