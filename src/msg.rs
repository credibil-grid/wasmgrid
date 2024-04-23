//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use futures::stream::{self, StreamExt};
use wasi_messaging::{self, RuntimeClient};

use crate::handler::{Client, HandlerProxy};

/// Start and run NATS for the specified wasm component.
pub async fn serve(handler: HandlerProxy, addr: String) -> anyhow::Result<()> {
    let msg_handler = handler.clone();

    // connect to NATS
    let client = Client::connect(addr).await?;

    // subscribe to channels
    let mut subscribers = vec![];
    for ch in &handler.channels().await? {
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let handler = msg_handler.clone();
        let client = client.clone();
        if let Err(e) = tokio::spawn(async move { handler.message(client, message).await }).await {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
