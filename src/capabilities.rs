//! # Capabilities
//!
//! This module contains runtime capability providers. Each capability is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

pub mod http;
pub mod jsondb;
pub mod keyvalue;
pub mod messaging;
pub mod p2p;
pub mod signature;
pub mod wrpc;
