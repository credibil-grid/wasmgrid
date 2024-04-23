#![feature(let_chains)]

#[allow(warnings)]
mod bindings;
pub mod http;
pub mod msg;

use wasi::exports::http::incoming_handler::Guest as HttpGuest;

use crate::bindings::exports::wasi::messaging::messaging_guest::Guest as MessagingGuest;

pub trait HttpMsgGuest: HttpGuest + MessagingGuest {}

struct GuestImpl;

wasi::http::proxy::export!(GuestImpl);
crate::bindings::export!(GuestImpl with_types_in crate::bindings);
