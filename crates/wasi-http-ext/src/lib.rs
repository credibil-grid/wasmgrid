#![feature(trait_alias)]

pub mod client;
pub mod request;

mod handler;
mod routing;
mod server;

pub use self::handler::*;
pub use self::request::*;
pub use self::routing::*;
pub use self::server::serve;
