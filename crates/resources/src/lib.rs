mod azkeyvault;
mod mongo;
mod nats;

use std::sync::{Arc, OnceLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Resources {
    nats: Arc<OnceLock<async_nats::Client>>,
    mongo: Arc<OnceLock<mongodb::Client>>,
    azkeyvault: Arc<OnceLock<azure_security_keyvault_keys::KeyClient>>,
}

impl Resources {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nats: Arc::new(OnceLock::new()),
            mongo: Arc::new(OnceLock::new()),
            azkeyvault: Arc::new(OnceLock::new()),
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

fn timeout<T>(once_lock: &OnceLock<T>, limit: u64) -> Result<&T>{
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
