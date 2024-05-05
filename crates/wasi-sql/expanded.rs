#![feature(prelude_import)]
//! # WASI SQL Host
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;
use crate::bindings::wasi::sql::readwrite;
use crate::bindings::wasi::sql::types::{Connection, Error, Row, Statement};
/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]
    pub struct Sql {}
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl Sql {
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: wasi::sql::types::Host + wasi::sql::readwrite::Host + Send,
                T: Send,
            {
                wasi::sql::types::add_to_linker(linker, get)?;
                wasi::sql::readwrite::add_to_linker(linker, get)?;
                Ok(())
            }
            /// Instantiates the provided `module` using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub async fn instantiate_async<T: Send>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = linker.instantiate_async(&mut store, component).await?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Instantiates a pre-instantiated module using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub async fn instantiate_pre<T: Send>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                instance_pre: &wasmtime::component::InstancePre<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = instance_pre.instantiate_async(&mut store).await?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Low-level creation wrapper for wrapping up the exports
            /// of the `instance` provided in this structure of wasm
            /// exports.
            ///
            /// This function will extract exports from the `instance`
            /// defined within `store` and wrap them all up in the
            /// returned structure which can be used to interact with
            /// the wasm module.
            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Self> {
                let mut store = store.as_context_mut();
                let mut exports = instance.exports(&mut store);
                let mut __exports = exports.root();
                Ok(Sql {})
            }
        }
    };
    pub mod wasi {
        pub mod sql {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                /// common data types
                #[component(variant)]
                pub enum DataType {
                    #[component(name = "int32")]
                    Int32(i32),
                    #[component(name = "int64")]
                    Int64(i64),
                    #[component(name = "uint32")]
                    Uint32(u32),
                    #[component(name = "uint64")]
                    Uint64(u64),
                    #[component(name = "float")]
                    Float(f64),
                    #[component(name = "double")]
                    Double(f64),
                    #[component(name = "str")]
                    Str(String),
                    #[component(name = "boolean")]
                    Boolean(bool),
                    #[component(name = "date")]
                    Date(String),
                    #[component(name = "time")]
                    Time(String),
                    #[component(name = "timestamp")]
                    Timestamp(String),
                    #[component(name = "binary")]
                    Binary(Vec<u8>),
                    #[component(name = "null")]
                    Null,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DataType {
                    #[inline]
                    fn clone(&self) -> DataType {
                        match self {
                            DataType::Int32(__self_0) => {
                                DataType::Int32(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Int64(__self_0) => {
                                DataType::Int64(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Uint32(__self_0) => {
                                DataType::Uint32(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Uint64(__self_0) => {
                                DataType::Uint64(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Float(__self_0) => {
                                DataType::Float(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Double(__self_0) => {
                                DataType::Double(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Str(__self_0) => {
                                DataType::Str(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Boolean(__self_0) => {
                                DataType::Boolean(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Date(__self_0) => {
                                DataType::Date(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Time(__self_0) => {
                                DataType::Time(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Timestamp(__self_0) => {
                                DataType::Timestamp(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Binary(__self_0) => {
                                DataType::Binary(::core::clone::Clone::clone(__self_0))
                            }
                            DataType::Null => DataType::Null,
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for DataType {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut std::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::Int32(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Int32)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[0usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Int64(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Int64)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[1usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Uint32(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(2u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Uint32)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[2usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Uint64(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(3u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Uint64)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[3usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Float(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(4u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Float)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[4usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Double(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(5u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Double)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[5usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Str(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(6u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Str)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[6usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Boolean(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(7u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Boolean)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[7usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Date(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(8u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Date)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[8usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Time(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(9u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Time)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[9usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Timestamp(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(10u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Timestamp)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[10usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Binary(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(11u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Binary)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .lower(
                                                    cx,
                                                    ty
                                                        .cases[11usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Null => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(12u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Null)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::Int32(value) => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[0usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Int64(value) => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[1usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Uint32(value) => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[2usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Uint64(value) => {
                                *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[3usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Float(value) => {
                                *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[4usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Double(value) => {
                                *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[5usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Str(value) => {
                                *cx.get::<1usize>(offset) = 6u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[6usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Boolean(value) => {
                                *cx.get::<1usize>(offset) = 7u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[7usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Date(value) => {
                                *cx.get::<1usize>(offset) = 8u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[8usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Time(value) => {
                                *cx.get::<1usize>(offset) = 9u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[9usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Timestamp(value) => {
                                *cx.get::<1usize>(offset) = 10u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[10usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Binary(value) => {
                                *cx.get::<1usize>(offset) = 11u8.to_le_bytes();
                                value
                                    .store(
                                        cx,
                                        ty
                                            .cases[11usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Null => {
                                *cx.get::<1usize>(offset) = 12u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for DataType {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => {
                                    Self::Int32(
                                        <i32 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[0usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Int32 },
                                        )?,
                                    )
                                }
                                1u32 => {
                                    Self::Int64(
                                        <i64 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[1usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Int64 },
                                        )?,
                                    )
                                }
                                2u32 => {
                                    Self::Uint32(
                                        <u32 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Uint32 },
                                        )?,
                                    )
                                }
                                3u32 => {
                                    Self::Uint64(
                                        <u64 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[3usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Uint64 },
                                        )?,
                                    )
                                }
                                4u32 => {
                                    Self::Float(
                                        <f64 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[4usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Float },
                                        )?,
                                    )
                                }
                                5u32 => {
                                    Self::Double(
                                        <f64 as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[5usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Double },
                                        )?,
                                    )
                                }
                                6u32 => {
                                    Self::Str(
                                        <String as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[6usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Str },
                                        )?,
                                    )
                                }
                                7u32 => {
                                    Self::Boolean(
                                        <bool as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[7usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Boolean },
                                        )?,
                                    )
                                }
                                8u32 => {
                                    Self::Date(
                                        <String as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[8usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Date },
                                        )?,
                                    )
                                }
                                9u32 => {
                                    Self::Time(
                                        <String as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[9usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Time },
                                        )?,
                                    )
                                }
                                10u32 => {
                                    Self::Timestamp(
                                        <String as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[10usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Timestamp },
                                        )?,
                                    )
                                }
                                11u32 => {
                                    Self::Binary(
                                        <Vec<
                                            u8,
                                        > as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[11usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Binary },
                                        )?,
                                    )
                                }
                                12u32 => Self::Null,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => {
                                    Self::Int32(
                                        <i32 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[0usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<i32 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                1u8 => {
                                    Self::Int64(
                                        <i64 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[1usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<i64 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                2u8 => {
                                    Self::Uint32(
                                        <u32 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<u32 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                3u8 => {
                                    Self::Uint64(
                                        <u64 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[3usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<u64 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                4u8 => {
                                    Self::Float(
                                        <f64 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[4usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<f64 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                5u8 => {
                                    Self::Double(
                                        <f64 as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[5usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<f64 as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                6u8 => {
                                    Self::Str(
                                        <String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[6usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                7u8 => {
                                    Self::Boolean(
                                        <bool as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[7usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<bool as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                8u8 => {
                                    Self::Date(
                                        <String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[8usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                9u8 => {
                                    Self::Time(
                                        <String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[9usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                10u8 => {
                                    Self::Timestamp(
                                        <String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[10usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                11u8 => {
                                    Self::Binary(
                                        <Vec<
                                            u8,
                                        > as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[11usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<Vec<
                                                u8,
                                            > as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                12u8 => Self::Null,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerDataType<
                        T0: Copy,
                        T1: Copy,
                        T2: Copy,
                        T3: Copy,
                        T4: Copy,
                        T5: Copy,
                        T6: Copy,
                        T7: Copy,
                        T8: Copy,
                        T9: Copy,
                        T10: Copy,
                        T11: Copy,
                    > {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadDataType<
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        >,
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                        T2: ::core::clone::Clone + Copy,
                        T3: ::core::clone::Clone + Copy,
                        T4: ::core::clone::Clone + Copy,
                        T5: ::core::clone::Clone + Copy,
                        T6: ::core::clone::Clone + Copy,
                        T7: ::core::clone::Clone + Copy,
                        T8: ::core::clone::Clone + Copy,
                        T9: ::core::clone::Clone + Copy,
                        T10: ::core::clone::Clone + Copy,
                        T11: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone
                    for LowerDataType<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
                        #[inline]
                        fn clone(
                            &self,
                        ) -> LowerDataType<
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        > {
                            LowerDataType {
                                tag: ::core::clone::Clone::clone(&self.tag),
                                payload: ::core::clone::Clone::clone(&self.payload),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                        T2: ::core::marker::Copy + Copy,
                        T3: ::core::marker::Copy + Copy,
                        T4: ::core::marker::Copy + Copy,
                        T5: ::core::marker::Copy + Copy,
                        T6: ::core::marker::Copy + Copy,
                        T7: ::core::marker::Copy + Copy,
                        T8: ::core::marker::Copy + Copy,
                        T9: ::core::marker::Copy + Copy,
                        T10: ::core::marker::Copy + Copy,
                        T11: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy
                    for LowerDataType<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadDataType<
                        T0: Copy,
                        T1: Copy,
                        T2: Copy,
                        T3: Copy,
                        T4: Copy,
                        T5: Copy,
                        T6: Copy,
                        T7: Copy,
                        T8: Copy,
                        T9: Copy,
                        T10: Copy,
                        T11: Copy,
                    > {
                        Int32: T0,
                        Int64: T1,
                        Uint32: T2,
                        Uint64: T3,
                        Float: T4,
                        Double: T5,
                        Str: T6,
                        Boolean: T7,
                        Date: T8,
                        Time: T9,
                        Timestamp: T10,
                        Binary: T11,
                        Null: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<
                        T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T3: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T4: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T5: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T6: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T7: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T8: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T9: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T10: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        T11: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    > ::core::clone::Clone
                    for LowerPayloadDataType<
                        T0,
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                    > {
                        #[inline]
                        fn clone(
                            &self,
                        ) -> LowerPayloadDataType<
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        > {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                        T2: ::core::marker::Copy + Copy,
                        T3: ::core::marker::Copy + Copy,
                        T4: ::core::marker::Copy + Copy,
                        T5: ::core::marker::Copy + Copy,
                        T6: ::core::marker::Copy + Copy,
                        T7: ::core::marker::Copy + Copy,
                        T8: ::core::marker::Copy + Copy,
                        T9: ::core::marker::Copy + Copy,
                        T10: ::core::marker::Copy + Copy,
                        T11: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy
                    for LowerPayloadDataType<
                        T0,
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                    > {}
                    unsafe impl wasmtime::component::ComponentType for DataType {
                        type Lower = LowerDataType<
                            <i32 as wasmtime::component::ComponentType>::Lower,
                            <i64 as wasmtime::component::ComponentType>::Lower,
                            <u32 as wasmtime::component::ComponentType>::Lower,
                            <u64 as wasmtime::component::ComponentType>::Lower,
                            <f64 as wasmtime::component::ComponentType>::Lower,
                            <f64 as wasmtime::component::ComponentType>::Lower,
                            <String as wasmtime::component::ComponentType>::Lower,
                            <bool as wasmtime::component::ComponentType>::Lower,
                            <String as wasmtime::component::ComponentType>::Lower,
                            <String as wasmtime::component::ComponentType>::Lower,
                            <String as wasmtime::component::ComponentType>::Lower,
                            <Vec<u8> as wasmtime::component::ComponentType>::Lower,
                        >;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_variant(
                                ty,
                                types,
                                &[
                                    (
                                        "int32",
                                        Some(<i32 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "int64",
                                        Some(<i64 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "uint32",
                                        Some(<u32 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "uint64",
                                        Some(<u64 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "float",
                                        Some(<f64 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "double",
                                        Some(<f64 as wasmtime::component::ComponentType>::typecheck),
                                    ),
                                    (
                                        "str",
                                        Some(
                                            <String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    (
                                        "boolean",
                                        Some(
                                            <bool as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    (
                                        "date",
                                        Some(
                                            <String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    (
                                        "time",
                                        Some(
                                            <String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    (
                                        "timestamp",
                                        Some(
                                            <String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    (
                                        "binary",
                                        Some(
                                            <Vec<u8> as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    ("null", None),
                                ],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[
                                Some(<i32 as wasmtime::component::ComponentType>::ABI),
                                Some(<i64 as wasmtime::component::ComponentType>::ABI),
                                Some(<u32 as wasmtime::component::ComponentType>::ABI),
                                Some(<u64 as wasmtime::component::ComponentType>::ABI),
                                Some(<f64 as wasmtime::component::ComponentType>::ABI),
                                Some(<f64 as wasmtime::component::ComponentType>::ABI),
                                Some(<String as wasmtime::component::ComponentType>::ABI),
                                Some(<bool as wasmtime::component::ComponentType>::ABI),
                                Some(<String as wasmtime::component::ComponentType>::ABI),
                                Some(<String as wasmtime::component::ComponentType>::ABI),
                                Some(<String as wasmtime::component::ComponentType>::ABI),
                                Some(<Vec<u8> as wasmtime::component::ComponentType>::ABI),
                                None,
                            ],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for DataType {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[
                            Some(<i32 as wasmtime::component::ComponentType>::ABI),
                            Some(<i64 as wasmtime::component::ComponentType>::ABI),
                            Some(<u32 as wasmtime::component::ComponentType>::ABI),
                            Some(<u64 as wasmtime::component::ComponentType>::ABI),
                            Some(<f64 as wasmtime::component::ComponentType>::ABI),
                            Some(<f64 as wasmtime::component::ComponentType>::ABI),
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                            Some(<bool as wasmtime::component::ComponentType>::ABI),
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                            Some(<Vec<u8> as wasmtime::component::ComponentType>::ABI),
                            None,
                        ];
                    }
                };
                impl core::fmt::Debug for DataType {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            DataType::Int32(e) => {
                                f.debug_tuple("DataType::Int32").field(e).finish()
                            }
                            DataType::Int64(e) => {
                                f.debug_tuple("DataType::Int64").field(e).finish()
                            }
                            DataType::Uint32(e) => {
                                f.debug_tuple("DataType::Uint32").field(e).finish()
                            }
                            DataType::Uint64(e) => {
                                f.debug_tuple("DataType::Uint64").field(e).finish()
                            }
                            DataType::Float(e) => {
                                f.debug_tuple("DataType::Float").field(e).finish()
                            }
                            DataType::Double(e) => {
                                f.debug_tuple("DataType::Double").field(e).finish()
                            }
                            DataType::Str(e) => {
                                f.debug_tuple("DataType::Str").field(e).finish()
                            }
                            DataType::Boolean(e) => {
                                f.debug_tuple("DataType::Boolean").field(e).finish()
                            }
                            DataType::Date(e) => {
                                f.debug_tuple("DataType::Date").field(e).finish()
                            }
                            DataType::Time(e) => {
                                f.debug_tuple("DataType::Time").field(e).finish()
                            }
                            DataType::Timestamp(e) => {
                                f.debug_tuple("DataType::Timestamp").field(e).finish()
                            }
                            DataType::Binary(e) => {
                                f.debug_tuple("DataType::Binary").field(e).finish()
                            }
                            DataType::Null => f.debug_tuple("DataType::Null").finish(),
                        }
                    }
                }
                const _: () = {
                    if !(16 == <DataType as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 16 == <DataType as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(8 == <DataType as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <DataType as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// one single row item
                #[component(record)]
                pub struct Row {
                    #[component(name = "field-name")]
                    pub field_name: String,
                    #[component(name = "value")]
                    pub value: DataType,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Row {
                    #[inline]
                    fn clone(&self) -> Row {
                        Row {
                            field_name: ::core::clone::Clone::clone(&self.field_name),
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for Row {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut std::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        wasmtime::component::Lower::lower(
                            &self.field_name,
                            cx,
                            ty.fields[0usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).field_name)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.value,
                            cx,
                            ty.fields[1usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).value)
                                    }
                                }
                            },
                        )?;
                        Ok(())
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        wasmtime::component::Lower::store(
                            &self.field_name,
                            cx,
                            ty.fields[0usize].ty,
                            <String as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.value,
                            cx,
                            ty.fields[1usize].ty,
                            <DataType as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for Row {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(Self {
                            field_name: <String as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[0usize].ty,
                                &src.field_name,
                            )?,
                            value: <DataType as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[1usize].ty,
                                &src.value,
                            )?,
                        })
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !((bytes.as_ptr() as usize)
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        let mut offset = 0;
                        Ok(Self {
                            field_name: <String as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<String as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            value: <DataType as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[1usize].ty,
                                &bytes[<DataType as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<DataType as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerRow<T0: Copy, T1: Copy> {
                        field_name: T0,
                        value: T1,
                        _align: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerRow<T0, T1> {
                        #[inline]
                        fn clone(&self) -> LowerRow<T0, T1> {
                            LowerRow {
                                field_name: ::core::clone::Clone::clone(&self.field_name),
                                value: ::core::clone::Clone::clone(&self.value),
                                _align: ::core::clone::Clone::clone(&self._align),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy for LowerRow<T0, T1> {}
                    unsafe impl wasmtime::component::ComponentType for Row {
                        type Lower = LowerRow<
                            <String as wasmtime::component::ComponentType>::Lower,
                            <DataType as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <String as wasmtime::component::ComponentType>::ABI,
                                <DataType as wasmtime::component::ComponentType>::ABI,
                            ],
                        );
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_record(
                                ty,
                                types,
                                &[
                                    (
                                        "field-name",
                                        <String as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "value",
                                        <DataType as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ],
                            )
                        }
                    }
                };
                impl core::fmt::Debug for Row {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("Row")
                            .field("field-name", &self.field_name)
                            .field("value", &self.value)
                            .finish()
                    }
                }
                const _: () = {
                    if !(24 == <Row as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <Row as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(8 == <Row as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <Row as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// allows parameterized queries
                /// e.g., prepare("SELECT * FROM users WHERE name = ? AND age = ?", vec!["John Doe", "32"])
                pub enum Statement {}
                pub trait HostStatement {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn prepare<'life0, 'async_trait>(
                        &'life0 mut self,
                        query: String,
                        params: Vec<String>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<
                                        wasmtime::component::Resource<Statement>,
                                        wasmtime::component::Resource<Error>,
                                    >,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Statement>,
                    ) -> wasmtime::Result<()>;
                }
                /// An error resource type.
                /// Currently, this provides only one function to return a string representation
                /// of the error. In the future, this will be extended to provide more information.
                pub enum Error {}
                pub trait HostError {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn trace<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Error>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<String>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Error>,
                    ) -> wasmtime::Result<()>;
                }
                /// A connection to a sql store.
                pub enum Connection {}
                pub trait HostConnection {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn open<'life0, 'async_trait>(
                        &'life0 mut self,
                        name: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<
                                        wasmtime::component::Resource<Connection>,
                                        wasmtime::component::Resource<Error>,
                                    >,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Connection>,
                    ) -> wasmtime::Result<()>;
                }
                pub trait Host: HostStatement + HostError + HostConnection {}
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                    U: Host + Send,
                {
                    let mut inst = linker.instance("wasi:sql/types@0.2.0-draft")?;
                    inst.resource(
                        "statement",
                        wasmtime::component::ResourceType::host::<Statement>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostStatement::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.resource(
                        "error",
                        wasmtime::component::ResourceType::host::<Error>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostError::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.resource(
                        "connection",
                        wasmtime::component::ResourceType::host::<Connection>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostConnection::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[static]statement.prepare",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0, arg1): (String, Vec<String>)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["module", "function"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"types" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[static]statement.prepare" as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    {};
                                    span
                                }
                            };
                            let _enter = span.enter();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "query", "params"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("call") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg0) as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg1) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = HostStatement::prepare(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "result"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("return") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&r) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok((r?,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]error.trace",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<Error>,)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["module", "function"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"types" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]error.trace" as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    {};
                                    span
                                }
                            };
                            let _enter = span.enter();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("call") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg0) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = HostError::trace(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "result"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("return") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&r) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok((r?,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[static]connection.open",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (String,)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["module", "function"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"types" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[static]connection.open" as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    {};
                                    span
                                }
                            };
                            let _enter = span.enter();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "name"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("call") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg0) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = HostConnection::open(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "result"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("return") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&r) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok((r?,))
                        }),
                    )?;
                    Ok(())
                }
            }
            #[allow(clippy::all)]
            pub mod readwrite {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Statement = super::super::super::wasi::sql::types::Statement;
                pub type Row = super::super::super::wasi::sql::types::Row;
                const _: () = {
                    if !(24 == <Row as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <Row as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(8 == <Row as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <Row as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub type Error = super::super::super::wasi::sql::types::Error;
                pub type Connection = super::super::super::wasi::sql::types::Connection;
                pub trait Host {
                    /// query is optimized for querying data, and
                    /// implementors can make use of that fact to optimize
                    /// the performance of query execution (e.g., using
                    /// indexes).
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn query<'life0, 'async_trait>(
                        &'life0 mut self,
                        c: wasmtime::component::Resource<Connection>,
                        q: wasmtime::component::Resource<Statement>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<Vec<Row>, wasmtime::component::Resource<Error>>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// exec is for modifying data in the database.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn exec<'life0, 'async_trait>(
                        &'life0 mut self,
                        c: wasmtime::component::Resource<Connection>,
                        q: wasmtime::component::Resource<Statement>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<u32, wasmtime::component::Resource<Error>>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                }
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                    U: Host + Send,
                {
                    let mut inst = linker.instance("wasi:sql/readwrite@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "query",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Connection>,
                                wasmtime::component::Resource<Statement>,
                            )|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["module", "function"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"readwrite" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"query" as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    {};
                                    span
                                }
                            };
                            let _enter = span.enter();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "c", "q"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("call") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg0) as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg1) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = Host::query(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "result"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("return") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&r) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok((r?,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "exec",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Connection>,
                                wasmtime::component::Resource<Statement>,
                            )|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["module", "function"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"readwrite" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"exec" as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    {};
                                    span
                                }
                            };
                            let _enter = span.enter();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "c", "q"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("call") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg0) as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg1) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = Host::exec(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-sql/src/lib.rs:13",
                                            "wasi_sql::bindings::wasi::sql::readwrite",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_sql::bindings::wasi::sql::readwrite",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "result"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = tracing::Level::TRACE
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && tracing::Level::TRACE
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("return") as &dyn Value,
                                                        ),
                                                    ),
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&r) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok((r?,))
                        }),
                    )?;
                    Ok(())
                }
            }
        }
    }
    const _: &str = "package wasi:sql@0.2.0-draft;\n\nworld imports {\n\timport readwrite;\n}";
    const _: &str = "interface readwrite {\n    use types.{statement, row, error, connection};\n    \n    /// query is optimized for querying data, and \n    /// implementors can make use of that fact to optimize \n    /// the performance of query execution (e.g., using\n    /// indexes).\n    query: func(c: borrow<connection>, q: borrow<statement>) -> result<list<row>, error>;\n    \n    /// exec is for modifying data in the database.\n    exec: func(c: borrow<connection>, q: borrow<statement>) -> result<u32, error>;\n}";
    const _: &str = "interface types {\n    /// one single row item\n    record row {\n        field-name: string,\n        value: data-type,\n    }\n    \n    /// common data types\n    variant data-type {\n        int32(s32),\n        int64(s64),\n        uint32(u32),\n        uint64(u64),\n        float(float64),\n        double(float64),\n        str(string),\n        boolean(bool),\n        date(string),\n        time(string),\n        timestamp(string),\n        binary(list<u8>),\n        null\n    }\n\n    /// allows parameterized queries\n    /// e.g., prepare(\"SELECT * FROM users WHERE name = ? AND age = ?\", vec![\"John Doe\", \"32\"])\n    resource statement {\n        prepare: static func(query: string, params: list<string>) -> result<statement, error>;\n    }\n    /// An error resource type.\n    /// Currently, this provides only one function to return a string representation\n    /// of the error. In the future, this will be extended to provide more information.\n    resource error {\n\t\ttrace: func() -> string;\n  \t}\n    \n    /// A connection to a sql store.\n    resource connection {\n        open: static func(name: string) -> result<connection, error>;\n    }\n}";
    const _: &str = "package component:wasi-sql;\n\nworld sql {\n\tinclude wasi:sql/imports@0.2.0-draft;\n}\n";
}
/// SqlView is implemented by the sql runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
pub trait SqlView: WasiView + Send {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        c: &'life1 Connection,
        q: &'life2 Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<Vec<Row>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait;
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn exec<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        c: &'life1 Connection,
        q: &'life2 Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<Vec<u32>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait;
}
impl<T: SqlView> readwrite::Host for T {
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn query<'life0, 'async_trait>(
        &'life0 mut self,
        c: Resource<Connection>,
        q: Resource<Statement>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = wasmtime::Result<Result<Vec<Row>, Resource<Error>>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                wasmtime::Result<Result<Vec<Row>, Resource<Error>>>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let mut __self = self;
            let c = c;
            let q = q;
            let __ret: wasmtime::Result<Result<Vec<Row>, Resource<Error>>> = {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event crates/wasi-sql/src/lib.rs:43",
                                "wasi_sql",
                                ::tracing::Level::DEBUG,
                                ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                ::core::option::Option::Some(43u32),
                                ::core::option::Option::Some("wasi_sql"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!("Host::sign") as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                ::core::panicking::panic("not yet implemented")
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn exec<'life0, 'async_trait>(
        &'life0 mut self,
        c: Resource<Connection>,
        q: Resource<Statement>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = wasmtime::Result<Result<u32, Resource<Error>>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                wasmtime::Result<Result<u32, Resource<Error>>>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let mut __self = self;
            let c = c;
            let q = q;
            let __ret: wasmtime::Result<Result<u32, Resource<Error>>> = {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event crates/wasi-sql/src/lib.rs:52",
                                "wasi_sql",
                                ::tracing::Level::DEBUG,
                                ::core::option::Option::Some("crates/wasi-sql/src/lib.rs"),
                                ::core::option::Option::Some(52u32),
                                ::core::option::Option::Some("wasi_sql"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!("Host::suite") as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                ::core::panicking::panic("not yet implemented")
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
