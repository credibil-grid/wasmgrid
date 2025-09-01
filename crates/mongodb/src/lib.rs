//! Azure Key Vault Secrets Client.

use std::collections::HashMap;
use std::env;

use anyhow::{Context, Result, anyhow};
use mongodb::Client;
use runtime::Resource;

#[derive(Default)]
pub struct MongoDb {
    attributes: HashMap<String, String>,
}

impl Resource<Client> for MongoDb {
    fn with_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    async fn connect(&self) -> Result<Client> {
        let uri = env::var("MONGODB_URI").context("fetching MONGODB_URI env var")?;

        let client = mongodb::Client::with_uri_str(uri).await.map_err(|e| {
            tracing::error!("failed to connect to mongo: {e}");
            anyhow!("failed to connect to mongo: {e}")
        })?;
        tracing::info!("connected to mongo");

        Ok(client)
    }
}
