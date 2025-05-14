#[cfg(feature = "vault")]
mod azkeyvault;
#[cfg(feature = "jsondb")]
mod mongo;
#[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
mod nats;

use std::sync::{Arc, OnceLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Result, anyhow};
#[cfg(feature = "vault")]
use azure_security_keyvault_keys::KeyClient;

#[derive(Clone)]
pub struct Resources {
    #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
    nats: Arc<OnceLock<async_nats::Client>>,
    #[cfg(feature = "jsondb")]
    mongo: Arc<OnceLock<mongodb::Client>>,
    #[cfg(feature = "vault")]
    azkeyvault: Arc<OnceLock<KeyClient>>,
}

impl Resources {
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
            nats: Arc::new(OnceLock::new()),
            #[cfg(feature = "jsondb")]
            mongo: Arc::new(OnceLock::new()),
            #[cfg(feature = "vault")]
            azkeyvault: Arc::new(OnceLock::new()),
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

fn timeout<T>(once_lock: &OnceLock<T>) -> Result<&T> {
    for _ in 0..10 {
        if let Some(client) = once_lock.get() {
            return Ok(client);
        }
        sleep(Duration::from_millis(10));
    }
    tracing::error!("failed to get resource");
    Err(anyhow!("failed to get resource"))
}
