//! # Capabilities
//!
//! This module contains runtime capability providers. Each capability is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

pub(crate) mod http;
pub(crate) mod keyvalue;
pub(crate) mod messaging;
pub(crate) mod signature;
pub(crate) mod sql;
