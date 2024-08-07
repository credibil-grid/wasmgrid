use std::sync::{Arc, Mutex};

use base64ct::{Base64UrlUnpadded, Encoding};

use super::azure::JsonWebKey;
use super::client::KeyVault;

/// Azure Key Vault key ring.
#[derive(Clone)]
pub struct AzureKeyRing {
    // Azure key vault client.
    pub client: KeyVault,
    // In-memory storage of key names before commits.
    key_buffer: Arc<Mutex<Vec<String>>>,
    // Namespace for keys in this key ring. This is used to differentiate keys in the same Azure
    // Key Vault instance to support, for example, multi-tenancy.
    namespace: Option<String>,
}

/// Azure Key Vault key ring constructor.
impl AzureKeyRing {
    /// Default constructor.
    ///
    /// # Arguments
    ///
    /// * `client` - Azure Key Vault client. Uses environment variables for credentials.
    /// * `namespace` - If multiple sets of keys can be stored in the same backing store, use a
    /// namespace to differentiate this key ring from adjacent ones. (eg. multi-tenancy)
    #[must_use]
    pub fn new(client: KeyVault, namespace: Option<String>) -> Self {
        Self {
            client,
            key_buffer: Arc::new(Mutex::new(Vec::new())),
            namespace,
        }
    }

    /// Delete a key from the key ring. This is a convenience method for testing.
    ///
    /// # Arguments
    ///
    /// * `op` - The key operation to delete.
    ///
    /// # Errors
    ///
    /// * `Err::RequestError` - A problem occurred calling the Azure Key Vault API.
    /// * `Err::InvalidConfig` - The Azure Key Vault URL or credentials are not configured.
    /// * `Err::AuthError` - A problem occurred authenticating with Azure Key Vault.
    pub async fn delete_key(&self, op: &KeyOperation) -> anyhow::Result<()> {
        let key_name = self.key_name(op);
        match self.client.delete_key(&key_name).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Key ring implementation backed by Azure Key Vault.
impl AzureKeyRing {
    /// Get the currently active public key for the specified key operation. If there is no such key
    /// attempt to find the most recent previous version.
    async fn active_key(&self, op: &KeyOperation) -> anyhow::Result<Jwk> {
        let key_name = self.key_name(op);

        // Get key from Azure Key Vault
        let key = self.client.get_key(&key_name, None).await?;

        // If the key is enabled, return it
        if key.attributes.enabled.is_some_and(|enabled| enabled) {
            return az_to_jwk(key.key);
        }

        // Otherwise the key is disabled, so return the immediate previous version
        let key = self.client.get_previous_version(&key_name).await?;

        az_to_jwk(key.key)
    }

    /// Create or get the next version of the key for the specified operation.
    async fn next_key(&self, op: &KeyOperation) -> anyhow::Result<Jwk> {
        // Create the key in Azure Key Vault or get the key if it already exists and the latest
        // version is disabled
        self.create_key(op, false).await
    }

    /// Commit (make active) the 'Next' key versions created.
    async fn commit(&self) -> anyhow::Result<()> {
        // activate each key in the key buffer
        let keys = {
            let kb = self
                .key_buffer
                .lock()
                .expect("lock on key buffer mutex poisoned due to panic in another thread");
            kb.clone()
        };
        for key_name in &keys {
            self.client.activate_key(key_name).await?;
        }
        let mut kb = self
            .key_buffer
            .lock()
            .expect("lock on key buffer mutex poisoned due to panic in another thread");
        kb.clear();
        Ok(())
    }
}

impl AzureKeyRing {
    // Creates a new key with the specified operation and active status. If the key already exists,
    // return the key without changing the active status.
    pub(crate) async fn create_key(&self, op: &KeyOperation, active: bool) -> anyhow::Result<Jwk> {
        let key_name = self.key_name(op);

        // Check if the key already exists
        match self.client.get_key(&key_name, None).await {
            Ok(key) => az_to_jwk(key.key),
            Err(e) => {
                if e.is(Err::KeyNotFound) {
                    // Create key in Azure Key Vault
                    let key_bundle = self.client.create_key(&key_name, active).await?;

                    // Add the key name to the key buffer
                    let mut kb = self
                        .key_buffer
                        .lock()
                        .expect("lock on key buffer mutex poisoned due to panic in another thread");
                    kb.push(key_name.clone());

                    // Return the key
                    az_to_jwk(key_bundle.key)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Constructs a conventional name for a key in the key store. Key names should be unique by
    /// operation and (optional) namespace.
    #[must_use]
    pub fn key_name(&self, op: &KeyOperation) -> String {
        match &self.namespace {
            Some(ns) => format!("vc-{op}-{ns}"),
            None => format!("vc-{op}"),
        }
    }
}

pub(crate) fn az_to_jwk(az_key: JsonWebKey) -> anyhow::Result<Jwk> {
    let crv = if az_key.curve_name.unwrap_or_default().as_str() == "P-256K" {
        "secp256k1"
    } else {
        // LATER: Add support for other curves.
        tracerr!(Err::InvalidKey, "unsupported curve.")
    };
    let x = if let Some(x) = az_key.x {
        Base64UrlUnpadded::encode_string(&x)
    } else {
        tracerr!(Err::InvalidKey, "Missing x coordinate.")
    };
    let y = if let Some(y) = az_key.y {
        Base64UrlUnpadded::encode_string(&y)
    } else {
        tracerr!(Err::InvalidKey, "Missing y coordinate.")
    };
    Ok(Jwk {
        kty: az_key.key_type,
        crv: Some(crv.to_string()),
        x: Some(x),
        y: Some(y),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key_ring() -> AzureKeyRing {
        let url = std::env::var("AZURE_KEY_VAULT").expect("AZURE_KEY_VAULT env var not set");
        let namespace = format!("test-{}", uuid::Uuid::new_v4());
        AzureKeyRing::new(KeyVault::new(url.as_str()), Some(namespace))
    }

    #[tokio::test]
    #[ignore]
    async fn active_key() {
        let kr = test_key_ring();

        let op = KeyOperation::Update;
        let key_name = kr.key_name(&op);

        // initially there should be no active key
        match kr.active_key(&op).await {
            Ok(_) => panic!("Expected key not found error from active_key"),
            Err(e) => {
                if !e.is(Err::KeyNotFound) {
                    panic!("Unexpected error from active_key: {}", e);
                }
            }
        };

        // create an active key, add it to the key buffer, and check it is returned without error
        let _new_key = match kr.create_key(&op, true).await {
            Ok(key) => key,
            Err(e) => {
                panic!("Error creating key: {}", e);
            }
        };

        let key_buffer = {
            let kb = kr
                .key_buffer
                .lock()
                .expect("lock on key buffer mutex poisoned due to panic in another thread");
            kb.clone()
        };
        assert!(key_buffer.contains(&key_name));
        let active_key = kr.active_key(&op).await.expect("active key should be Ok");

        // pause to allow Azure Key Vault to create another version with different time stamp since
        // resolution is only to the second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // create a next key, add it to the key buffer, and check it is returned without error
        let next_key = kr.next_key(&op).await;
        assert!(next_key.is_ok());

        // calling active_key after next_key should result in the same key version (idempotency)
        let active_key2 = kr.active_key(&op).await.expect("expected active key should be Ok");
        assert_eq!(active_key.x, active_key2.x);
        assert_eq!(active_key.y, active_key2.y);

        // clean up
        for key_name in key_buffer.iter() {
            kr.client
                .delete_key(key_name)
                .await
                .expect("key should have been deleted without error");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn next_key() {
        let kr = test_key_ring();
        let op = KeyOperation::Update;

        // create new key
        let next = kr.next_key(&op).await.expect("failed to get next update key");

        // should be tracking 1 created key
        let key_buffer = {
            let kb = kr.key_buffer.lock().expect("lock on key buffer mutex poisoned");
            kb.clone()
        };
        assert_eq!(key_buffer.len(), 1);

        // key should not be active
        match kr.active_key(&op).await {
            Ok(_) => panic!("Expected key not found error from active_key"),
            Err(e) => {
                if !e.is(Err::KeyNotFound) {
                    panic!("Unexpected error from active_key: {}", e);
                }
            }
        };

        // check key properties
        assert_eq!(next.kty, "EC");
        assert_eq!(next.crv, Some("secp256k1".to_string()));

        // another call to next_key should return the same key
        let next2 = kr.next_key(&op).await.expect("failed to get next expected update key");
        assert_eq!(next.x, next2.x);
        assert_eq!(next.y, next2.y);

        // clean up
        for key_name in key_buffer.iter() {
            kr.client.delete_key(key_name).await.expect("failed to delete key");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn commit() {
        let kr = test_key_ring();

        // create a new key
        let op = KeyOperation::Update;
        let key_name = kr.key_name(&op);
        let next = kr.next_key(&op).await.expect("failed to get next update key");

        // key should not be active
        match kr.active_key(&op).await {
            Ok(_) => panic!("Expected key not found error from active_key"),
            Err(e) => {
                if !e.is(Err::KeyNotFound) {
                    panic!("Unexpected error from active_key: {}", e);
                }
            }
        };

        // expect one key that can be committed
        let key_buffer = {
            let kb = kr.key_buffer.lock().expect("lock on key buffer mutex poisoned");
            kb.clone()
        };
        assert_eq!(key_buffer.len(), 1);

        // commit
        let commit = kr.commit().await;
        assert!(commit.is_ok());

        // expect no keys for committment
        let key_buffer = {
            let kb = kr.key_buffer.lock().expect("lock on key buffer mutex poisoned");
            kb.clone()
        };
        assert_eq!(key_buffer.len(), 0);

        // key should be active
        let active2 = kr.active_key(&op).await.expect("failed to get next update key");

        // next key and currently active key should have the same version
        assert_eq!(next.x, active2.x);
        assert_eq!(next.y, active2.y);

        // clean up
        kr.client.delete_key(&key_name).await.expect("failed to delete key");
    }
}
