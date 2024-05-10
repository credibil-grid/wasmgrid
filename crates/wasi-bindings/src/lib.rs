//! # WASI Bindings
//!
//! This module generates and exports WASI Guest bindings for local wit worlds.
//! The bindings are exported in as similar a manner to those in the Bytecode
//! Alliance's [wasi] crate.
//!
//! [wasi]: https://github.com/bytecodealliance/wasi

mod docdb_bindings {
    wit_bindgen::generate!({
        world: "docdb",
    });
}

/// Bindings for the `wasi:signature` world.
pub mod docdb {
    pub use crate::docdb_bindings::wasi::docdb::*;
}

mod keyvalue_bindings {
    wit_bindgen::generate!({
        world: "keyvalue",
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:keyvalue` world.
/// See (<https://github.com/WebAssembly/wasi-keyvalue/>)
pub mod keyvalue {
    pub use crate::keyvalue_bindings::export;
    pub use crate::keyvalue_bindings::wasi::keyvalue::*;

    pub mod exports {
        pub use crate::keyvalue_bindings::exports::wasi;
        pub use crate::keyvalue_bindings::exports::wasi::keyvalue::*;
    }
}

mod messaging_bindings {
    wit_bindgen::generate!({
        world: "messaging",
        pub_export_macro: true
    });
}

/// Bindings for the `wasi:messaging` world.
/// See (<https://github.com/WebAssembly/wasi-messaging/>)
pub mod messaging {
    pub use crate::messaging_bindings::export;
    pub use crate::messaging_bindings::wasi::messaging::*;

    pub mod exports {
        pub use crate::messaging_bindings::exports::wasi;
        pub use crate::messaging_bindings::exports::wasi::messaging::*;
    }
}

mod signature_bindings {
    wit_bindgen::generate!({
        world: "signature",
    });
}

/// Bindings for the `wasi:signature` world.
pub mod signature {
    pub use crate::signature_bindings::wasi::signature::*;
}

mod sql_bindings {
    wit_bindgen::generate!({
        world: "sql",
    });
}

/// Bindings for the `wasi:sql` world.
pub mod sql {
    pub use crate::sql_bindings::wasi::sql::*;
}
