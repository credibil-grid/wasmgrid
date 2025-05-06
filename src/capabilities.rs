//! # Capabilities
//!
//! This module contains runtime capability providers. Each capability is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "jsondb")]
pub mod jsondb;
#[cfg(feature = "keyvalue")]
pub mod keyvalue;
#[cfg(feature = "messaging")]
pub mod messaging;
// #[cfg(feature = "p2p")]
// pub mod p2p;
#[cfg(feature = "rpc")]
pub mod rpc;
// #[cfg(feature = "vault")]
// pub mod vault;
