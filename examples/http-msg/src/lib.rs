#![feature(let_chains)]

#[allow(warnings)]
mod bindings;
mod http;
mod messaging;

use crate::messaging::Messaging;
use crate::http::Http;

// export guest implementations
wasi::http::proxy::export!(Http);
crate::bindings::export!(Messaging with_types_in crate::bindings);
