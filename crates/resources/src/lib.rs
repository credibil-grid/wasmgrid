#[cfg(feature = "azkeyvault")]
mod azkeyvault;
#[cfg(feature = "mongodb")]
mod mongo;
#[cfg(feature = "nats")]
mod nats;

use std::sync::{Arc, OnceLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Resources {
    #[cfg(feature = "azkeyvault")]
    azkeyvault: Arc<OnceLock<azure_security_keyvault_secrets::SecretClient>>,
    #[cfg(feature = "mongodb")]
    mongo: Arc<OnceLock<mongodb::Client>>,
    #[cfg(feature = "nats")]
    nats: Arc<OnceLock<async_nats::Client>>,
}

impl Resources {
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "azkeyvault")]
            azkeyvault: Arc::new(OnceLock::new()),
            #[cfg(feature = "mongodb")]
            mongo: Arc::new(OnceLock::new()),
            #[cfg(feature = "nats")]
            nats: Arc::new(OnceLock::new()),
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

fn timeout<T>(once_lock: &OnceLock<T>, limit: u64) -> Result<&T> {
    let duration = Duration::from_millis(limit / 10);
    for _ in 0..10 {
        if let Some(client) = once_lock.get() {
            return Ok(client);
        }
        sleep(duration);
    }
    tracing::error!("failed to get resource");
    Err(anyhow!("failed to get resource"))
}
