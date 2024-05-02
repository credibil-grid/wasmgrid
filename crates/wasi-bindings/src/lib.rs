mod messaging_bindings {
    wit_bindgen::generate!({
        world: "messaging",
        pub_export_macro: true
    });
}

/// Export bindings in a similar manner to the [`wasi`](https://github.com/bytecodealliance/wasi)
/// API.
pub mod messaging {
    pub use crate::messaging_bindings::export;
    pub use crate::messaging_bindings::wasi::messaging::*;

    pub mod exports {
        pub use crate::messaging_bindings::exports::wasi;
        pub use crate::messaging_bindings::exports::wasi::messaging::*;
    }
}
