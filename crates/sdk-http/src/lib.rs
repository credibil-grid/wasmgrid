//! # HTTP SDK
//!
//! Wasm component (guest) HTTP SDK.

mod client;
mod error;
mod router;
mod uri;

pub use axum;

pub use self::client::*;
pub use self::error::*;
pub use self::router::serve;
pub use self::uri::*;
