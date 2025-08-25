//! # WASI Bindings
//!
//! This module generates and exports WASI Guest bindings for local wit worlds.
//! The bindings are exported in as similar a manner to those in the Bytecode
//! Alliance's [wasi] crate.
//!
//! [wasi]: https://github.com/bytecodealliance/wasi

/// Bindings for the `wasi:blobstore` world.
/// See (<https://github.com/WebAssembly/wasi-blobstore/>)
pub mod blobstore {
    pub use self::wasi::blobstore::*;

    wit_bindgen::generate!({
        world: "blobstore",
        path: "../../wit",
        generate_all,
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:keyvalue` world.
/// See (<https://github.com/WebAssembly/wasi-keyvalue/>)
pub mod keyvalue {
    pub use self::wasi::keyvalue::*;

    wit_bindgen::generate!({
        world: "keyvalue",
        path: "../../wit",
        generate_all,
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:messaging` world.
/// See (<https://github.com/WebAssembly/wasi-messaging/>)
pub mod messaging {
    pub use self::exports::wasi::messaging::*;
    pub use self::wasi::messaging::*;

    wit_bindgen::generate!({
        world: "messaging",
        path: "../../wit",
        generate_all,
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:otel` world.
pub mod otel {
    #![allow(clippy::collection_is_never_read)]

    pub use self::wasi::otel::*;

    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:vault` world.
pub mod vault {
    pub use self::wasi::vault::*;

    wit_bindgen::generate!({
        world: "vault",
        path: "../../wit",
        generate_all,
        pub_export_macro: true
    });
}
