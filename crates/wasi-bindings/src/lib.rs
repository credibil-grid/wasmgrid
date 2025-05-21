#![allow(clippy::missing_safety_doc)]

//! # WASI Bindings
//!
//! This module generates and exports WASI Guest bindings for local wit worlds.
//! The bindings are exported in as similar a manner to those in the Bytecode
//! Alliance's [wasi] crate.
//!
//! [wasi]: https://github.com/bytecodealliance/wasi

mod jsondb_bindings {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "jsondb",
        with: {
            "wasi:jsondb/readwrite@0.1.0-draft": generate,
            "wasi:jsondb/types@0.1.0-draft": generate
        }
    });
}

/// Bindings for the `wasi:vault` world.
pub mod jsondb {
    pub use crate::jsondb_bindings::wasi::jsondb::*;
}

mod keyvalue_bindings {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "keyvalue",
        with: {
            "wasi:keyvalue/store@0.2.0-draft": generate,
            "wasi:keyvalue/atomics@0.2.0-draft": generate,
            "wasi:keyvalue/batch@0.2.0-draft": generate,
            "wasi:keyvalue/watcher@0.2.0-draft": generate
        },
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
        path: "../../wit",
        world: "messaging",
        with: {
            "wasi:messaging/incoming-handler@0.2.0-draft": generate,
            "wasi:messaging/producer@0.2.0-draft": generate,
            "wasi:messaging/types@0.2.0-draft": generate,
        },
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

// /// Bindings for the `wasi:p2p` world.
// mod p2p_bindings {
//     wit_bindgen::generate!({
//         path: "../../wit",
//         world: "p2p",
//         with: {
//             "wasi:p2p/types@0.1.0-draft": generate,
//             "wasi:p2p/container@0.1.0-draft": generate,
//         },
//         additional_derives: [
//             serde::Serialize,
//             serde::Deserialize,
//         ]
//     });
// }

// pub mod p2p {
//     pub use crate::p2p_bindings::wasi::p2p::*;
// }

/// Bindings for the `wasi:wrpc` world.
mod rpc_bindings {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "rpc",
        with: {
            "wasi:rpc/types@0.1.0-draft": generate,
            "wasi:rpc/client@0.1.0-draft": generate,
            "wasi:rpc/server@0.1.0-draft": generate,
        },
        pub_export_macro: true,
    });
}

pub mod rpc {
    pub use crate::rpc_bindings::export;
    pub use crate::rpc_bindings::wasi::rpc::*;
    pub mod exports {
        pub use crate::rpc_bindings::exports::wasi;
        pub use crate::rpc_bindings::exports::wasi::rpc::server::*;
    }
}

mod vault_bindings {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "vault",
        with: {
            "wasi:vault/keystore@0.1.0-draft": generate,
        },
        additional_derives: [
            Clone,
        ],
    });
}

/// Bindings for the `wasi:vault` world.
pub mod vault {
    pub use crate::vault_bindings::wasi::vault::*;
}
