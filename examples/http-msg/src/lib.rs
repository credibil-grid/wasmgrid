#![feature(let_chains)]

#[allow(warnings)]
mod bindings;
mod http;
mod messaging;

use crate::http::Http;
use crate::messaging::Messaging;

// export guest implementations
wasi::http::proxy::export!(Http);
crate::bindings::export!(Messaging with_types_in crate::bindings);
