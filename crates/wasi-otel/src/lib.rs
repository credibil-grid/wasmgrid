//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification
//! (wasi:otel) in the context of WebAssembly System Interface (WASI)
//! components.

mod metrics;
mod tracing;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use self::wasi::otel::types::Error;

    wasmtime::component::bindgen!({
        world: "otel",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        trappable_error_type: {
            "wasi:otel/types/error" => Error,
        },
    });
}

use std::marker::PhantomData;

use anyhow::Result;
use credibil_otel::init;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::SpanExporter as _;
use runtime::Linkable;
use wasi_core::Ctx;
use wasmtime::component::{HasData, Linker};

use self::generated::wasi::otel as wasi_otel;

pub struct Otel<'a> {
    exporter: SpanExporter,
    _phantom: PhantomData<&'a ()>,
}

impl Otel<'_> {
    fn new(_: &mut Ctx) -> Otel<'_> {
        let mut exporter =
            SpanExporter::builder().with_tonic().build().expect("should build exporter");
        exporter.set_resource(init::resource());

        Otel {
            exporter,
            _phantom: PhantomData,
        }
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Otel<'a>;
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add the `wasi-otel` world's interfaces to a [`Linker`]
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        wasi_otel::tracing::add_to_linker::<_, Data>(linker, Otel::new)?;
        wasi_otel::metrics::add_to_linker::<_, Data>(linker, Otel::new)?;
        wasi_otel::types::add_to_linker::<_, Data>(linker, Otel::new)
    }
}
