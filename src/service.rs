//! # Service
//!
//! This module contains runtime service providers. Each service is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "keyvalue")]
pub mod keyvalue;
// #[cfg(feature = "messaging")]
// pub mod messaging;
// #[cfg(feature = "rpc")]
// pub mod rpc;

// #[cfg(feature = "jsondb")]
// pub mod jsondb;
// #[cfg(feature = "vault")]
// pub mod vault;
