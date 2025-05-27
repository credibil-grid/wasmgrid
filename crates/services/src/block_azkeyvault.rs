//! # WASI Block Store Service
//! 
//! This module implements a runtime service for `wasi:block` using Azure Key
//! Vault.
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use self::wasi::blockstore::types::Error;
    pub use super::{Block, Identifier};

    wasmtime::component::bindgen!({
        world: "blockstore",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:blockstore/types/block": Block,
            "wasi:blockstore/types/identifier": Identifier,
        },
        trappable_error_type: {
            "wasi:blockstore/types/error" => Error,
        },
    });
}

use anyhow::anyhow;

use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::blockstore;
use self::generated::wasi::blockstore::types::{Error, Location, Metadata};
use crate::{Ctx, Resources};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct BlockHost<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl BlockHost<'_> {
    const fn new(c: &mut Ctx) -> BlockHost<'_> {
        BlockHost {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

#[derive(Clone)]
pub struct Block {
    identifier: Identifier,
    value: Vec<u8>,
}

#[derive(Clone)]
pub struct Identifier {
    location: Location,
    version: String,
    metadata: Option<Metadata>,
}

pub struct Service;