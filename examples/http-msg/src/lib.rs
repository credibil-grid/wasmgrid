#![feature(let_chains)]

mod http;
mod messaging;

use crate::http::Http;
use crate::messaging::Messaging;

// export guest implementations
wasi::http::proxy::export!(Http);
wasi_bindings::messaging::export!(Messaging with_types_in wasi_bindings::messaging);
