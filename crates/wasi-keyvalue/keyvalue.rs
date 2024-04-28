#![feature(prelude_import)]
//! # WASI KeyValue Host
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod atomics {
    use wasmtime::component::Resource;
    use crate::bindings::wasi::keyvalue::atomics;
    use crate::bindings::wasi::keyvalue::atomics::Bucket;
    use crate::bindings::wasi::keyvalue::store::Error;
    use crate::KeyValueView;
    impl<T: KeyValueView> atomics::Host for T {
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
        fn increment<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            key: String,
            delta: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<Result<u64, Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<Result<u64, Error>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let key = key;
                let delta = delta;
                let __ret: wasmtime::Result<Result<u64, Error>> = {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!("implement increment"),
                            ),
                        );
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
mod batch {
    use wasmtime::component::Resource;
    use crate::bindings::wasi::keyvalue::batch;
    use crate::bindings::wasi::keyvalue::store::{self, Bucket};
    use crate::KeyValueView;
    impl<T: KeyValueView> batch::Host for T {
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
        fn get_many<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            keys: Vec<String>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        Result<Vec<Option<(String, Vec<u8>)>>, store::Error>,
                        wasmtime::Error,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<
                        Result<Vec<Option<(String, Vec<u8>)>>, store::Error>,
                        wasmtime::Error,
                    >,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let keys = keys;
                let __ret: Result<
                    Result<Vec<Option<(String, Vec<u8>)>>, store::Error>,
                    wasmtime::Error,
                > = { ::core::panicking::panic("not yet implemented") };
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
        fn set_many<'life0, 'async_trait>(
            &'life0 mut self,
            __arg1: Resource<Bucket>,
            __arg2: Vec<(String, Vec<u8>)>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Result<(), store::Error>, wasmtime::Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Result<(), store::Error>, wasmtime::Error>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let __arg1 = __arg1;
                let __arg2 = __arg2;
                let __ret: Result<Result<(), store::Error>, wasmtime::Error> = {
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
        fn delete_many<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            keys: Vec<String>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Result<(), store::Error>, wasmtime::Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Result<(), store::Error>, wasmtime::Error>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let keys = keys;
                let __ret: Result<Result<(), store::Error>, wasmtime::Error> = {
                    ::core::panicking::panic("not yet implemented")
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
mod store {
    use wasmtime::component::Resource;
    use crate::bindings::wasi::keyvalue::store::{self, Bucket, KeyResponse};
    use crate::KeyValueView;
    impl<T: KeyValueView> store::Host for T {
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
        fn open<'life0, 'async_trait>(
            &'life0 mut self,
            identifier: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<Result<Resource<Bucket>, store::Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<Result<Resource<Bucket>, store::Error>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let identifier = identifier;
                let __ret: wasmtime::Result<Result<Resource<Bucket>, store::Error>> = {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!("implement open"),
                            ),
                        );
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl<T: KeyValueView> store::HostBucket for T {
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
        fn get<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            key: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<Result<Option<Vec<u8>>, store::Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<Result<Option<Vec<u8>>, store::Error>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let key = key;
                let __ret: wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> = {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!("implement open"),
                            ),
                        );
                    }
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
        fn set<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            key: String,
            value: Vec<u8>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<Result<(), store::Error>, wasmtime::Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<Result<(), store::Error>, wasmtime::Error>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let key = key;
                let value = value;
                let __ret: wasmtime::Result<Result<(), store::Error>, wasmtime::Error> = {
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
        fn delete<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            key: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Result<(), store::Error>, wasmtime::Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Result<(), store::Error>, wasmtime::Error>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let key = key;
                let __ret: Result<Result<(), store::Error>, wasmtime::Error> = {
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
        fn exists<'life0, 'async_trait>(
            &'life0 mut self,
            bucket: Resource<Bucket>,
            key: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<Result<bool, store::Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<Result<bool, store::Error>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bucket = bucket;
                let key = key;
                let __ret: wasmtime::Result<Result<bool, store::Error>> = {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!("implement open"),
                            ),
                        );
                    }
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
        fn list_keys<'life0, 'async_trait>(
            &'life0 mut self,
            __arg1: Resource<Bucket>,
            __arg2: Option<u64>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Result<KeyResponse, store::Error>, wasmtime::Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Result<KeyResponse, store::Error>, wasmtime::Error>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let __arg1 = __arg1;
                let __arg2 = __arg2;
                let __ret: Result<Result<KeyResponse, store::Error>, wasmtime::Error> = {
                    ::core::panicking::panic("not yet implemented")
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn drop(&mut self, bucket: Resource<Bucket>) -> Result<(), wasmtime::Error> {
            ::core::panicking::panic("not yet implemented")
        }
    }
}
use wasmtime_wasi::WasiView;
/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    pub use anyhow::Error;
    pub struct Keyvalue {
        interface0: exports::wasi::keyvalue::watcher::Guest,
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl Keyvalue {
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: wasi::keyvalue::store::Host + wasi::keyvalue::atomics::Host
                    + wasi::keyvalue::batch::Host + Send,
                T: Send,
            {
                wasi::keyvalue::store::add_to_linker(linker, get)?;
                wasi::keyvalue::atomics::add_to_linker(linker, get)?;
                wasi::keyvalue::batch::add_to_linker(linker, get)?;
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
                let interface0 = exports::wasi::keyvalue::watcher::Guest::new(
                    &mut __exports
                        .instance("wasi:keyvalue/watcher@0.2.0-draft")
                        .ok_or_else(|| ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!(
                                    "exported instance `wasi:keyvalue/watcher@0.2.0-draft` not present",
                                ),
                            );
                            error
                        }))?,
                )?;
                Ok(Keyvalue { interface0 })
            }
            pub fn wasi_keyvalue_watcher(
                &self,
            ) -> &exports::wasi::keyvalue::watcher::Guest {
                &self.interface0
            }
        }
    };
    pub mod wasi {
        pub mod keyvalue {
            #[allow(clippy::all)]
            pub mod store {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                /// The set of errors which may be raised by functions in this package
                #[component(variant)]
                pub enum Error {
                    /// The host does not recognize the store identifier requested.
                    #[component(name = "no-such-store")]
                    NoSuchStore,
                    /// The requesting component does not have access to the specified store
                    /// (which may or may not exist).
                    #[component(name = "access-denied")]
                    AccessDenied,
                    /// Some implementation-specific error has occurred (e.g. I/O)
                    #[component(name = "other")]
                    Other(String),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Error {
                    #[inline]
                    fn clone(&self) -> Error {
                        match self {
                            Error::NoSuchStore => Error::NoSuchStore,
                            Error::AccessDenied => Error::AccessDenied,
                            Error::Other(__self_0) => {
                                Error::Other(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for Error {
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
                            Self::NoSuchStore => {
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
                                                    m.map(|p| &raw mut (*p).NoSuchStore)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::AccessDenied => {
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
                                                    m.map(|p| &raw mut (*p).AccessDenied)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Other(value) => {
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
                                                    m.map(|p| &raw mut (*p).Other)
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
                            Self::NoSuchStore => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::AccessDenied => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Other(value) => {
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
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for Error {
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
                                0u32 => Self::NoSuchStore,
                                1u32 => Self::AccessDenied,
                                2u32 => {
                                    Self::Other(
                                        <String as wasmtime::component::Lift>::lift(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.Other },
                                        )?,
                                    )
                                }
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
                                0u8 => Self::NoSuchStore,
                                1u8 => Self::AccessDenied,
                                2u8 => {
                                    Self::Other(
                                        <String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
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
                    pub struct LowerError<T2: Copy> {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadError<T2>,
                    }
                    #[automatically_derived]
                    impl<T2: ::core::clone::Clone + Copy> ::core::clone::Clone
                    for LowerError<T2> {
                        #[inline]
                        fn clone(&self) -> LowerError<T2> {
                            LowerError {
                                tag: ::core::clone::Clone::clone(&self.tag),
                                payload: ::core::clone::Clone::clone(&self.payload),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                    for LowerError<T2> {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadError<T2: Copy> {
                        NoSuchStore: [wasmtime::ValRaw; 0],
                        AccessDenied: [wasmtime::ValRaw; 0],
                        Other: T2,
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<
                        T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerPayloadError<T2> {
                        #[inline]
                        fn clone(&self) -> LowerPayloadError<T2> {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                    for LowerPayloadError<T2> {}
                    unsafe impl wasmtime::component::ComponentType for Error {
                        type Lower = LowerError<
                            <String as wasmtime::component::ComponentType>::Lower,
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
                                    ("no-such-store", None),
                                    ("access-denied", None),
                                    (
                                        "other",
                                        Some(
                                            <String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                ],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[
                                None,
                                None,
                                Some(<String as wasmtime::component::ComponentType>::ABI),
                            ],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for Error {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[
                            None,
                            None,
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                        ];
                    }
                };
                impl core::fmt::Debug for Error {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Error::NoSuchStore => {
                                f.debug_tuple("Error::NoSuchStore").finish()
                            }
                            Error::AccessDenied => {
                                f.debug_tuple("Error::AccessDenied").finish()
                            }
                            Error::Other(e) => {
                                f.debug_tuple("Error::Other").field(e).finish()
                            }
                        }
                    }
                }
                impl core::fmt::Display for Error {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl std::error::Error for Error {}
                const _: () = {
                    if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// A response to a `list-keys` operation.
                #[component(record)]
                pub struct KeyResponse {
                    /// The list of keys returned by the query.
                    #[component(name = "keys")]
                    pub keys: Vec<String>,
                    /// The continuation token to use to fetch the next page of keys. If this is `null`, then
                    /// there are no more keys to fetch.
                    #[component(name = "cursor")]
                    pub cursor: Option<u64>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for KeyResponse {
                    #[inline]
                    fn clone(&self) -> KeyResponse {
                        KeyResponse {
                            keys: ::core::clone::Clone::clone(&self.keys),
                            cursor: ::core::clone::Clone::clone(&self.cursor),
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for KeyResponse {
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
                            &self.keys,
                            cx,
                            ty.fields[0usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).keys)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.cursor,
                            cx,
                            ty.fields[1usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).cursor)
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
                            &self.keys,
                            cx,
                            ty.fields[0usize].ty,
                            <Vec<String> as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.cursor,
                            cx,
                            ty.fields[1usize].ty,
                            <Option<u64> as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for KeyResponse {
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
                            keys: <Vec<
                                String,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[0usize].ty,
                                &src.keys,
                            )?,
                            cursor: <Option<
                                u64,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[1usize].ty,
                                &src.cursor,
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
                            keys: <Vec<
                                String,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<Vec<
                                    String,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Vec<
                                    String,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            cursor: <Option<
                                u64,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[1usize].ty,
                                &bytes[<Option<
                                    u64,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Option<
                                    u64,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerKeyResponse<T0: Copy, T1: Copy> {
                        keys: T0,
                        cursor: T1,
                        _align: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerKeyResponse<T0, T1> {
                        #[inline]
                        fn clone(&self) -> LowerKeyResponse<T0, T1> {
                            LowerKeyResponse {
                                keys: ::core::clone::Clone::clone(&self.keys),
                                cursor: ::core::clone::Clone::clone(&self.cursor),
                                _align: ::core::clone::Clone::clone(&self._align),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy for LowerKeyResponse<T0, T1> {}
                    unsafe impl wasmtime::component::ComponentType for KeyResponse {
                        type Lower = LowerKeyResponse<
                            <Vec<String> as wasmtime::component::ComponentType>::Lower,
                            <Option<u64> as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <Vec<String> as wasmtime::component::ComponentType>::ABI,
                                <Option<u64> as wasmtime::component::ComponentType>::ABI,
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
                                        "keys",
                                        <Vec<
                                            String,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "cursor",
                                        <Option<
                                            u64,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ],
                            )
                        }
                    }
                };
                impl core::fmt::Debug for KeyResponse {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("KeyResponse")
                            .field("keys", &self.keys)
                            .field("cursor", &self.cursor)
                            .finish()
                    }
                }
                const _: () = {
                    if !(24
                        == <KeyResponse as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <KeyResponse as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(8
                        == <KeyResponse as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <KeyResponse as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// A bucket is a collection of key-value pairs. Each key-value pair is stored as a entry in the
                /// bucket, and the bucket itself acts as a collection of all these entries.
                ///
                /// It is worth noting that the exact terminology for bucket in key-value stores can very
                /// depending on the specific implementation. For example:
                ///
                /// 1. Amazon DynamoDB calls a collection of key-value pairs a table
                /// 2. Redis has hashes, sets, and sorted sets as different types of collections
                /// 3. Cassandra calls a collection of key-value pairs a column family
                /// 4. MongoDB calls a collection of key-value pairs a collection
                /// 5. Riak calls a collection of key-value pairs a bucket
                /// 6. Memcached calls a collection of key-value pairs a slab
                /// 7. Azure Cosmos DB calls a collection of key-value pairs a container
                ///
                /// In this interface, we use the term `bucket` to refer to a collection of key-value pairs
                pub enum Bucket {}
                pub trait HostBucket {
                    /// Get the value associated with the specified `key`
                    ///
                    /// The value is returned as an option. If the key-value pair exists in the
                    /// store, it returns `Ok(value)`. If the key does not exist in the
                    /// store, it returns `Ok(none)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn get<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<Option<Vec<u8>>, Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Set the value associated with the key in the store. If the key already
                    /// exists in the store, it overwrites the value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn set<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: String,
                        value: Vec<u8>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<(), Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Delete the key-value pair associated with the key in the store.
                    ///
                    /// If the key does not exist in the store, it does nothing.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn delete<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<(), Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Check if the key exists in the store.
                    ///
                    /// If the key exists in the store, it returns `Ok(true)`. If the key does
                    /// not exist in the store, it returns `Ok(false)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn exists<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<bool, Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Get all the keys in the store with an optional cursor (for use in pagination). It
                    /// returns a list of keys. Please note that for most KeyValue implementations, this is a
                    /// can be a very expensive operation and so it should be used judiciously. Implementations
                    /// can return any number of keys in a single response, but they should never attempt to
                    /// send more data than is reasonable (i.e. on a small edge device, this may only be a few
                    /// KB, while on a large machine this could be several MB). Any response should also return
                    /// a cursor that can be used to fetch the next page of keys. See the `key-response` record
                    /// for more information.
                    ///
                    /// Note that the keys are not guaranteed to be returned in any particular order.
                    ///
                    /// If the store is empty, it returns an empty list.
                    ///
                    /// MAY show an out-of-date list of keys if there are concurrent writes to the store.
                    ///
                    /// If any error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn list_keys<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        cursor: Option<u64>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<KeyResponse, Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Bucket>,
                    ) -> wasmtime::Result<()>;
                }
                pub trait Host: HostBucket {
                    /// Get the bucket with the specified identifier.
                    ///
                    /// `identifier` must refer to a bucket provided by the host.
                    ///
                    /// `error::no-such-store` will be raised if the `identifier` is not recognized.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn open<'life0, 'async_trait>(
                        &'life0 mut self,
                        identifier: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<wasmtime::component::Resource<Bucket>, Error>,
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
                    let mut inst = linker.instance("wasi:keyvalue/store@0.2.0-draft")?;
                    inst.resource(
                        "bucket",
                        wasmtime::component::ResourceType::host::<Bucket>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostBucket::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "open",
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
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"open" as &dyn Value),
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "identifier"],
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
                            let r = Host::open(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                        "[method]bucket.get",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, String)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]bucket.get" as &dyn Value,
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_", "key"],
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
                            let r = HostBucket::get(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                        "[method]bucket.set",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (wasmtime::component::Resource<Bucket>, String, Vec<u8>)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]bucket.set" as &dyn Value,
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_", "key", "value"],
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
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg2) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = HostBucket::set(host, arg0, arg1, arg2).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                        "[method]bucket.delete",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, String)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]bucket.delete" as &dyn Value,
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_", "key"],
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
                            let r = HostBucket::delete(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                        "[method]bucket.exists",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, String)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]bucket.exists" as &dyn Value,
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_", "key"],
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
                            let r = HostBucket::exists(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                        "[method]bucket.list-keys",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, Option<u64>)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
                                                            ::core::option::Option::Some(&"store" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[method]bucket.list-keys" as &dyn Value,
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "self_", "cursor"],
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
                            let r = HostBucket::list_keys(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::store",
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
            pub mod atomics {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Bucket = super::super::super::wasi::keyvalue::store::Bucket;
                pub type Error = super::super::super::wasi::keyvalue::store::Error;
                const _: () = {
                    if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub trait Host {
                    /// Atomically increment the value associated with the key in the store by the given delta. It
                    /// returns the new value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair with the value set
                    /// to the given delta.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn increment<'life0, 'async_trait>(
                        &'life0 mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key: String,
                        delta: u64,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<u64, Error>>,
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
                    let mut inst = linker.instance("wasi:keyvalue/atomics@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "increment",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (wasmtime::component::Resource<Bucket>, String, u64)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
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
                                                            ::core::option::Option::Some(&"atomics" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"increment" as &dyn Value),
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "bucket", "key", "delta"],
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
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &tracing::field::debug(&arg2) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = Host::increment(host, arg0, arg1, arg2).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::atomics",
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
            pub mod batch {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Bucket = super::super::super::wasi::keyvalue::store::Bucket;
                pub type Error = super::super::super::wasi::keyvalue::store::Error;
                const _: () = {
                    if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub trait Host {
                    /// Get the key-value pairs associated with the keys in the store. It returns a list of
                    /// key-value pairs.
                    ///
                    /// If any of the keys do not exist in the store, it returns a `none` value for that pair in the
                    /// list.
                    ///
                    /// MAY show an out-of-date value if there are concurrent writes to the store.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn get_many<'life0, 'async_trait>(
                        &'life0 mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: Vec<String>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<Vec<Option<(String, Vec<u8>)>>, Error>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Set the values associated with the keys in the store. If the key already exists in the
                    /// store, it overwrites the value.
                    ///
                    /// Note that the key-value pairs are not guaranteed to be set in the order they are provided.
                    ///
                    /// If any of the keys do not exist in the store, it creates a new key-value pair.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not
                    /// rollback the key-value pairs that were already set. Thus, this batch operation does not
                    /// guarantee atomicity, implying that some key-value pairs could be set while others might
                    /// fail.
                    ///
                    /// Other concurrent operations may also be able to see the partial results.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn set_many<'life0, 'async_trait>(
                        &'life0 mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key_values: Vec<(String, Vec<u8>)>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<(), Error>>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Delete the key-value pairs associated with the keys in the store.
                    ///
                    /// Note that the key-value pairs are not guaranteed to be deleted in the order they are
                    /// provided.
                    ///
                    /// If any of the keys do not exist in the store, it skips the key.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not
                    /// rollback the key-value pairs that were already deleted. Thus, this batch operation does not
                    /// guarantee atomicity, implying that some key-value pairs could be deleted while others might
                    /// fail.
                    ///
                    /// Other concurrent operations may also be able to see the partial results.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn delete_many<'life0, 'async_trait>(
                        &'life0 mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: Vec<String>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<Result<(), Error>>,
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
                    let mut inst = linker.instance("wasi:keyvalue/batch@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "get-many",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, Vec<String>)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
                                                            ::core::option::Option::Some(&"batch" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"get-many" as &dyn Value),
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "bucket", "keys"],
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
                            let r = Host::get_many(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
                        "set-many",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                Vec<(String, Vec<u8>)>,
                            )|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
                                                            ::core::option::Option::Some(&"batch" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"set-many" as &dyn Value),
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "bucket", "key_values"],
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
                            let r = Host::set_many(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
                        "delete-many",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, Vec<String>)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
                                                            ::core::option::Option::Some(&"batch" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"delete-many" as &dyn Value),
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
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "bucket", "keys"],
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
                            let r = Host::delete_many(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-keyvalue/src/lib.rs:13",
                                            "wasi_keyvalue::bindings::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::wasi::keyvalue::batch",
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
    pub mod exports {
        pub mod wasi {
            pub mod keyvalue {
                #[allow(clippy::all)]
                pub mod watcher {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    pub type Bucket = super::super::super::super::wasi::keyvalue::store::Bucket;
                    pub struct Guest {
                        on_set: wasmtime::component::Func,
                        on_delete: wasmtime::component::Func,
                    }
                    impl Guest {
                        pub fn new(
                            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                        ) -> wasmtime::Result<Guest> {
                            let on_set = *__exports
                                .typed_func::<
                                    (wasmtime::component::Resource<Bucket>, &str, &[u8]),
                                    (),
                                >("on-set")?
                                .func();
                            let on_delete = *__exports
                                .typed_func::<
                                    (wasmtime::component::Resource<Bucket>, &str),
                                    (),
                                >("on-delete")?
                                .func();
                            Ok(Guest { on_set, on_delete })
                        }
                        /// Handle the `set` event for the given bucket and key. It includes a reference to the `bucket`
                        /// that can be used to interact with the store.
                        pub async fn call_on_set<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::Resource<Bucket>,
                            arg1: &str,
                            arg2: &[u8],
                        ) -> wasmtime::Result<()>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "wasi_keyvalue::bindings::exports::wasi::keyvalue::watcher",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::exports::wasi::keyvalue::watcher",
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
                                                            ::core::option::Option::Some(
                                                                &"wasi:keyvalue/watcher@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"on-set" as &dyn Value),
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
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<Bucket>, &str, &[u8]),
                                    (),
                                >::new_unchecked(self.on_set)
                            };
                            let () = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(())
                        }
                        /// Handle the `delete` event for the given bucket and key. It includes a reference to the
                        /// `bucket` that can be used to interact with the store.
                        pub async fn call_on_delete<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::Resource<Bucket>,
                            arg1: &str,
                        ) -> wasmtime::Result<()>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "wasi_keyvalue::bindings::exports::wasi::keyvalue::watcher",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-keyvalue/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(13u32),
                                            ::core::option::Option::Some(
                                                "wasi_keyvalue::bindings::exports::wasi::keyvalue::watcher",
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
                                                            ::core::option::Option::Some(
                                                                &"wasi:keyvalue/watcher@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"on-delete" as &dyn Value),
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
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<Bucket>, &str),
                                    (),
                                >::new_unchecked(self.on_delete)
                            };
                            let () = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }
    const _: &str = "/// A keyvalue interface that provides eventually consistent key-value operations.\n/// \n/// Each of these operations acts on a single key-value pair.\n/// \n/// The value in the key-value pair is defined as a `u8` byte array and the intention is that it is\n/// the common denominator for all data types defined by different key-value stores to handle data,\n/// ensuring compatibility between different key-value stores. Note: the clients will be expecting\n/// serialization/deserialization overhead to be handled by the key-value store. The value could be\n/// a serialized object from JSON, HTML or vendor-specific data types like AWS S3 objects.\n/// \n/// Data consistency in a key value store refers to the guarantee that once a write operation\n/// completes, all subsequent read operations will return the value that was written.\n/// \n/// Any implementation of this interface must have enough consistency to guarantee \"reading your\n/// writes.\" In particular, this means that the client should never get a value that is older than\n/// the one it wrote, but it MAY get a newer value if one was written around the same time. These\n/// guarantees only apply to the same client (which will likely be provided by the host or an\n/// external capability of some kind). In this context a \"client\" is referring to the caller or\n/// guest that is consuming this interface. Once a write request is committed by a specific client,\n/// all subsequent read requests by the same client will reflect that write or any subsequent\n/// writes. Another client running in a different context may or may not immediately see the result\n/// due to the replication lag. As an example of all of this, if a value at a given key is A, and\n/// the client writes B, then immediately reads, it should get B. If something else writes C in\n/// quick succession, then the client may get C. However, a client running in a separate context may\n/// still see A or B\ninterface store {\n    /// The set of errors which may be raised by functions in this package\n    variant error {\n        /// The host does not recognize the store identifier requested.\n        no-such-store,\n\n        /// The requesting component does not have access to the specified store\n        /// (which may or may not exist).\n        access-denied,\n\n        /// Some implementation-specific error has occurred (e.g. I/O)\n        other(string)\n    }\n\n    /// A response to a `list-keys` operation.\n    record key-response {\n        /// The list of keys returned by the query.\n        keys: list<string>,\n        /// The continuation token to use to fetch the next page of keys. If this is `null`, then\n        /// there are no more keys to fetch.\n        cursor: option<u64>\n    }\n\n    /// Get the bucket with the specified identifier.\n    ///\n    /// `identifier` must refer to a bucket provided by the host.\n    ///\n    /// `error::no-such-store` will be raised if the `identifier` is not recognized.\n    open: func(identifier: string) -> result<bucket, error>;\n\n    /// A bucket is a collection of key-value pairs. Each key-value pair is stored as a entry in the\n    /// bucket, and the bucket itself acts as a collection of all these entries.\n    ///\n    /// It is worth noting that the exact terminology for bucket in key-value stores can very\n    /// depending on the specific implementation. For example:\n    ///\n    /// 1. Amazon DynamoDB calls a collection of key-value pairs a table\n    /// 2. Redis has hashes, sets, and sorted sets as different types of collections\n    /// 3. Cassandra calls a collection of key-value pairs a column family\n    /// 4. MongoDB calls a collection of key-value pairs a collection\n    /// 5. Riak calls a collection of key-value pairs a bucket\n    /// 6. Memcached calls a collection of key-value pairs a slab\n    /// 7. Azure Cosmos DB calls a collection of key-value pairs a container\n    ///\n    /// In this interface, we use the term `bucket` to refer to a collection of key-value pairs\n    resource bucket {\n        /// Get the value associated with the specified `key`\n        ///\n        /// The value is returned as an option. If the key-value pair exists in the\n        /// store, it returns `Ok(value)`. If the key does not exist in the\n        /// store, it returns `Ok(none)`. \n        ///\n        /// If any other error occurs, it returns an `Err(error)`.\n        get: func(key: string) -> result<option<list<u8>>, error>;\n\n        /// Set the value associated with the key in the store. If the key already\n        /// exists in the store, it overwrites the value.\n        ///\n        /// If the key does not exist in the store, it creates a new key-value pair.\n        /// \n        /// If any other error occurs, it returns an `Err(error)`.\n        set: func(key: string, value: list<u8>) -> result<_, error>;\n\n        /// Delete the key-value pair associated with the key in the store.\n        /// \n        /// If the key does not exist in the store, it does nothing.\n        ///\n        /// If any other error occurs, it returns an `Err(error)`.\n        delete: func(key: string) -> result<_, error>;\n\n        /// Check if the key exists in the store.\n        /// \n        /// If the key exists in the store, it returns `Ok(true)`. If the key does\n        /// not exist in the store, it returns `Ok(false)`.\n        /// \n        /// If any other error occurs, it returns an `Err(error)`.\n        exists: func(key: string) -> result<bool, error>;\n\n        /// Get all the keys in the store with an optional cursor (for use in pagination). It\n        /// returns a list of keys. Please note that for most KeyValue implementations, this is a\n        /// can be a very expensive operation and so it should be used judiciously. Implementations\n        /// can return any number of keys in a single response, but they should never attempt to\n        /// send more data than is reasonable (i.e. on a small edge device, this may only be a few\n        /// KB, while on a large machine this could be several MB). Any response should also return\n        /// a cursor that can be used to fetch the next page of keys. See the `key-response` record\n        /// for more information.\n        /// \n        /// Note that the keys are not guaranteed to be returned in any particular order.\n        /// \n        /// If the store is empty, it returns an empty list.\n        /// \n        /// MAY show an out-of-date list of keys if there are concurrent writes to the store.\n        /// \n        /// If any error occurs, it returns an `Err(error)`.\n        list-keys: func(cursor: option<u64>) -> result<key-response, error>;\n    }\n}\n";
    const _: &str = "package wasi:keyvalue@0.2.0-draft;\n\n/// The `wasi:keyvalue/imports` world provides common APIs for interacting with key-value stores.\n/// Components targeting this world will be able to do:\n/// \n/// 1. CRUD (create, read, update, delete) operations on key-value stores.\n/// 2. Atomic `increment` and CAS (compare-and-swap) operations.\n/// 3. Batch operations that can reduce the number of round trips to the network.\nworld imports {\n\t/// The `store` capability allows the component to perform eventually consistent operations on\n\t/// the key-value store.\n\timport store;\n\n\t/// The `atomic` capability allows the component to perform atomic / `increment` and CAS\n\t/// (compare-and-swap) operations.\n\timport atomics;\n\n\t/// The `batch` capability allows the component to perform eventually consistent batch\n\t/// operations that can reduce the number of round trips to the network.\n\timport batch;\n}\n\nworld watch-service {\n\tinclude imports;\n\texport watcher;\n}";
    const _: &str = "/// A keyvalue interface that provides batch operations.\n/// \n/// A batch operation is an operation that operates on multiple keys at once.\n/// \n/// Batch operations are useful for reducing network round-trip time. For example, if you want to\n/// get the values associated with 100 keys, you can either do 100 get operations or you can do 1\n/// batch get operation. The batch operation is faster because it only needs to make 1 network call\n/// instead of 100.\n/// \n/// A batch operation does not guarantee atomicity, meaning that if the batch operation fails, some\n/// of the keys may have been modified and some may not. \n/// \n/// This interface does has the same consistency guarantees as the `store` interface, meaning that\n/// you should be able to \"read your writes.\"\n/// \n/// Please note that this interface is bare functions that take a reference to a bucket. This is to\n/// get around the current lack of a way to \"extend\" a resource with additional methods inside of\n/// wit. Future version of the interface will instead extend these methods on the base `bucket`\n/// resource.\ninterface batch {\n    use store.{bucket, error};\n\n    /// Get the key-value pairs associated with the keys in the store. It returns a list of\n    /// key-value pairs.\n    ///\n    /// If any of the keys do not exist in the store, it returns a `none` value for that pair in the\n    /// list.\n    /// \n    /// MAY show an out-of-date value if there are concurrent writes to the store.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`.\n    get-many: func(bucket: borrow<bucket>, keys: list<string>) -> result<list<option<tuple<string, list<u8>>>>, error>;\n\n    /// Set the values associated with the keys in the store. If the key already exists in the\n    /// store, it overwrites the value. \n    /// \n    /// Note that the key-value pairs are not guaranteed to be set in the order they are provided. \n    ///\n    /// If any of the keys do not exist in the store, it creates a new key-value pair.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not\n    /// rollback the key-value pairs that were already set. Thus, this batch operation does not\n    /// guarantee atomicity, implying that some key-value pairs could be set while others might\n    /// fail. \n    /// \n    /// Other concurrent operations may also be able to see the partial results.\n    set-many: func(bucket: borrow<bucket>, key-values: list<tuple<string, list<u8>>>) -> result<_, error>;\n\n    /// Delete the key-value pairs associated with the keys in the store.\n    /// \n    /// Note that the key-value pairs are not guaranteed to be deleted in the order they are\n    /// provided.\n    /// \n    /// If any of the keys do not exist in the store, it skips the key.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not\n    /// rollback the key-value pairs that were already deleted. Thus, this batch operation does not\n    /// guarantee atomicity, implying that some key-value pairs could be deleted while others might\n    /// fail.\n    /// \n    /// Other concurrent operations may also be able to see the partial results.\n    delete-many: func(bucket: borrow<bucket>, keys: list<string>) -> result<_, error>;\n}\n";
    const _: &str = "/// A keyvalue interface that provides watch operations.\n/// \n/// This interface is used to provide event-driven mechanisms to handle\n/// keyvalue changes.\ninterface watcher {\n\t/// A keyvalue interface that provides handle-watch operations.\n\tuse store.{bucket};\n\n\t/// Handle the `set` event for the given bucket and key. It includes a reference to the `bucket`\n\t/// that can be used to interact with the store.\n\ton-set: func(bucket: bucket, key: string, value: list<u8>);\n\n\t/// Handle the `delete` event for the given bucket and key. It includes a reference to the\n\t/// `bucket` that can be used to interact with the store.\n\ton-delete: func(bucket: bucket, key: string);\n}";
    const _: &str = "/// A keyvalue interface that provides atomic operations.\n/// \n/// Atomic operations are single, indivisible operations. When a fault causes an atomic operation to\n/// fail, it will appear to the invoker of the atomic operation that the action either completed\n/// successfully or did nothing at all.\n/// \n/// Please note that this interface is bare functions that take a reference to a bucket. This is to\n/// get around the current lack of a way to \"extend\" a resource with additional methods inside of\n/// wit. Future version of the interface will instead extend these methods on the base `bucket`\n/// resource.\ninterface atomics {\n  \tuse store.{bucket, error};\n\n  \t/// Atomically increment the value associated with the key in the store by the given delta. It\n\t/// returns the new value.\n\t///\n\t/// If the key does not exist in the store, it creates a new key-value pair with the value set\n\t/// to the given delta. \n\t///\n\t/// If any other error occurs, it returns an `Err(error)`.\n\tincrement: func(bucket: borrow<bucket>, key: string, delta: u64) -> result<u64, error>;\n}";
    const _: &str = "package component:wasi-keyvalue;\n\nworld imports {\n\timport wasi:keyvalue/store@0.2.0-draft;\n\timport wasi:keyvalue/atomics@0.2.0-draft;\n\timport wasi:keyvalue/batch@0.2.0-draft;\n}\n\nworld keyvalue {\n\tinclude imports;\n\texport wasi:keyvalue/watcher@0.2.0-draft;\n}\n";
}
/// KeyValueView is implemented by the keyvalue runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
pub trait KeyValueView: WasiView + Send {}
