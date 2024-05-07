//! # WASI Messaging Host

pub mod consumer;
pub mod producer;

use std::any::Any;

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

pub type Client = Box<dyn RuntimeClient>;
// pub type Error = Box<dyn RuntimeError>;
pub type Error = anyhow::Error;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Client, Error};

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:messaging/messaging-types/client": Client,
            "wasi:messaging/messaging-types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:messaging/messaging-types/error" => Error,
        // },
    });
}

// pub use crate::bindings::exports;
use crate::bindings::wasi::messaging::messaging_types::{
    self, GuestConfiguration, HostClient, HostError,
};

// Type T — the host — is provided by the messaging runtime.
impl<T: ClientView + ErrorView> messaging_types::Host for T {
    // fn convert_error(&mut self, e: anyhow::Error) -> anyhow::Result<Error> {
    //     todo!()
    // }
}

/// MessageView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[async_trait::async_trait]
pub trait ClientView: WasiView + Send {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>>;

    async fn update_configuration(&mut self, gc: GuestConfiguration) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: ClientView + ErrorView> HostClient for T {
    /// Connect to the runtime's messaging server.
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        tracing::debug!("HostClient::connect {name}");
        Ok(Ok(T::connect(self, name).await?))
    }

    /// Drop the specified client resource.
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        tracing::debug!("HostClient::drop");
        self.table().delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait ErrorView: WasiView + Send {
    /// Return a string representation of the error.
    async fn trace(&mut self) -> anyhow::Result<String>;

    /// Drop the specified error resource.
    ///
    /// # Errors
    fn drop(&mut self, err: Resource<Error>) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: ErrorView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        Ok(T::trace(self).await?)
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        T::drop(self, rep)
    }
}

/// RuntimeClient is implemented by the runtime to provide this host with access
/// to runtime functionality.
#[async_trait::async_trait]
pub trait RuntimeClient: Sync + Send {
    fn as_any(&self) -> &dyn Any;
}

// /// RuntimeError is implemented by the runtime to provide the host with access
// /// to runtime error functionality.
// #[async_trait::async_trait]
// pub trait RuntimeError: Sync + Send {
//     fn as_any(&self) -> &dyn Any;
// }
