#![feature(prelude_import)]
//! # WASI Messaging Host
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod consumer {
    use futures::stream::StreamExt;
    use tokio::time::{sleep, Duration};
    use wasmtime::component::Resource;
    use super::bindings::consumer;
    use super::bindings::messaging_types::{Client, Error, GuestConfiguration, Message};
    use crate::MessagingView;
    impl<T: MessagingView> consumer::Host for T {
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
        fn subscribe_try_receive<'life0, 'async_trait>(
            &'life0 mut self,
            client: Resource<Client>,
            ch: String,
            t_milliseconds: u32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<
                        anyhow::Result<Option<Vec<Message>>, Resource<Error>>,
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
                    wasmtime::Result<
                        anyhow::Result<Option<Vec<Message>>, Resource<Error>>,
                    >,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let client = client;
                let ch = ch;
                let t_milliseconds = t_milliseconds;
                let __ret: wasmtime::Result<
                    anyhow::Result<Option<Vec<Message>>, Resource<Error>>,
                > = {
                    let client = __self.table().get(&client)?;
                    let mut subscriber = client.subscribe(ch).await?;
                    let stream = subscriber
                        .by_ref()
                        .take_until(
                            sleep(Duration::from_millis(u64::from(t_milliseconds))),
                        );
                    let messages = stream.collect().await;
                    Ok(Ok(Some(messages)))
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
        fn subscribe_receive<'life0, 'async_trait>(
            &'life0 mut self,
            client: Resource<Client>,
            ch: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<
                        anyhow::Result<Vec<Message>, Resource<Error>>,
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
                    wasmtime::Result<anyhow::Result<Vec<Message>, Resource<Error>>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let client = client;
                let ch = ch;
                let __ret: wasmtime::Result<
                    anyhow::Result<Vec<Message>, Resource<Error>>,
                > = {
                    let client = __self.table().get(&client)?;
                    let mut subscriber = client.subscribe(ch).await?;
                    let messages = subscriber.by_ref().take(1).collect().await;
                    Ok(Ok(messages))
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
        fn update_guest_configuration<'life0, 'async_trait>(
            &'life0 mut self,
            gc: GuestConfiguration,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let gc = gc;
                let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                    Ok(__self.update_configuration(gc).await)
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
        fn complete_message<'life0, 'async_trait>(
            &'life0 mut self,
            msg: Message,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let msg = msg;
                let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                    {
                        ::std::io::_print(
                            format_args!(
                                "TODO: implement complete_message: {0:?}\n",
                                msg.metadata,
                            ),
                        );
                    };
                    Ok(Ok(()))
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
        fn abandon_message<'life0, 'async_trait>(
            &'life0 mut self,
            msg: Message,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let msg = msg;
                let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                    {
                        ::std::io::_print(
                            format_args!(
                                "TODO: implement abandon_message: {0:?}\n",
                                msg.metadata,
                            ),
                        );
                    };
                    Ok(Ok(()))
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
mod producer {
    use bytes::Bytes;
    use wasmtime::component::Resource;
    use super::bindings::messaging_types::{Client, Error, Message};
    use super::bindings::producer;
    use crate::MessagingView;
    impl<T: MessagingView> producer::Host for T {
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
        fn send<'life0, 'async_trait>(
            &'life0 mut self,
            client: Resource<Client>,
            ch: String,
            messages: Vec<Message>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    wasmtime::Result<anyhow::Result<(), Resource<Error>>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let client = client;
                let ch = ch;
                let messages = messages;
                let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                    let client = __self.table().get(&client)?;
                    for m in messages {
                        let data = Bytes::from(m.data.clone());
                        client.publish(ch.clone(), data).await?;
                    }
                    Ok(Ok(()))
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
use std::pin::Pin;
use bytes::Bytes;
use futures::stream::Stream;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;
use crate::bindings::messaging_types::{
    self, Error, GuestConfiguration, HostClient, HostError, Message,
};
pub type Client = Box<dyn RuntimeClient>;
pub type Subscriber = Pin<Box<dyn RuntimeSubscriber>>;
/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    pub use anyhow::Error;
    pub use wasi::messaging::*;
    pub use super::Client;
    pub struct Messaging {
        interface0: exports::wasi::messaging::messaging_guest::Guest,
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl Messaging {
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: wasi::messaging::messaging_types::Host
                    + wasi::messaging::producer::Host + wasi::messaging::consumer::Host
                    + Send,
                T: Send,
            {
                wasi::messaging::messaging_types::add_to_linker(linker, get)?;
                wasi::messaging::producer::add_to_linker(linker, get)?;
                wasi::messaging::consumer::add_to_linker(linker, get)?;
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
                let interface0 = exports::wasi::messaging::messaging_guest::Guest::new(
                    &mut __exports
                        .instance("wasi:messaging/messaging-guest@0.2.0-draft")
                        .ok_or_else(|| ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!(
                                    "exported instance `wasi:messaging/messaging-guest@0.2.0-draft` not present",
                                ),
                            );
                            error
                        }))?,
                )?;
                Ok(Messaging { interface0 })
            }
            pub fn wasi_messaging_messaging_guest(
                &self,
            ) -> &exports::wasi::messaging::messaging_guest::Guest {
                &self.interface0
            }
        }
    };
    pub mod wasi {
        pub mod messaging {
            #[allow(clippy::all)]
            pub mod messaging_types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                /// A connection to a message-exchange service (e.g., buffer, broker, etc.).
                pub use super::super::super::Client as Client;
                pub trait HostClient {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn connect<'life0, 'async_trait>(
                        &'life0 mut self,
                        name: String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<
                                        wasmtime::component::Resource<Client>,
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
                        rep: wasmtime::component::Resource<Client>,
                    ) -> wasmtime::Result<()>;
                }
                /// TODO(danbugs): This should be eventually extracted as an underlying type for other wasi-cloud-core interfaces.
                pub use super::super::super::Error as Error;
                pub trait HostError {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn trace<'life0, 'async_trait>(
                        &'life0 mut self,
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
                /// There are two types of channels:
                /// - publish-subscribe channel, which is a broadcast channel, and
                /// - point-to-point channel, which is a unicast channel.
                ///
                /// The interface doesn't highlight this difference in the type itself as that's uniquely a consumer issue.
                pub type Channel = String;
                const _: () = {
                    if !(8 == <Channel as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <Channel as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Channel as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Channel as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// Configuration includes a required list of channels the guest is subscribing to, and an optional list of extensions key-value pairs
                /// (e.g., partitions/offsets to read from in Kafka/EventHubs, QoS etc.).
                #[component(record)]
                pub struct GuestConfiguration {
                    #[component(name = "channels")]
                    pub channels: Vec<Channel>,
                    #[component(name = "extensions")]
                    pub extensions: Option<Vec<(String, String)>>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for GuestConfiguration {
                    #[inline]
                    fn clone(&self) -> GuestConfiguration {
                        GuestConfiguration {
                            channels: ::core::clone::Clone::clone(&self.channels),
                            extensions: ::core::clone::Clone::clone(&self.extensions),
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for GuestConfiguration {
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
                            &self.channels,
                            cx,
                            ty.fields[0usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).channels)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.extensions,
                            cx,
                            ty.fields[1usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).extensions)
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
                            &self.channels,
                            cx,
                            ty.fields[0usize].ty,
                            <Vec<Channel> as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.extensions,
                            cx,
                            ty.fields[1usize].ty,
                            <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for GuestConfiguration {
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
                            channels: <Vec<
                                Channel,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[0usize].ty,
                                &src.channels,
                            )?,
                            extensions: <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[1usize].ty,
                                &src.extensions,
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
                            channels: <Vec<
                                Channel,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<Vec<
                                    Channel,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Vec<
                                    Channel,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            extensions: <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[1usize].ty,
                                &bytes[<Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerGuestConfiguration<T0: Copy, T1: Copy> {
                        channels: T0,
                        extensions: T1,
                        _align: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerGuestConfiguration<T0, T1> {
                        #[inline]
                        fn clone(&self) -> LowerGuestConfiguration<T0, T1> {
                            LowerGuestConfiguration {
                                channels: ::core::clone::Clone::clone(&self.channels),
                                extensions: ::core::clone::Clone::clone(&self.extensions),
                                _align: ::core::clone::Clone::clone(&self._align),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy for LowerGuestConfiguration<T0, T1> {}
                    unsafe impl wasmtime::component::ComponentType
                    for GuestConfiguration {
                        type Lower = LowerGuestConfiguration<
                            <Vec<Channel> as wasmtime::component::ComponentType>::Lower,
                            <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <Vec<Channel> as wasmtime::component::ComponentType>::ABI,
                                <Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::ABI,
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
                                        "channels",
                                        <Vec<
                                            Channel,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "extensions",
                                        <Option<
                                            Vec<(String, String)>,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ],
                            )
                        }
                    }
                };
                impl core::fmt::Debug for GuestConfiguration {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("GuestConfiguration")
                            .field("channels", &self.channels)
                            .field("extensions", &self.extensions)
                            .finish()
                    }
                }
                const _: () = {
                    if !(20
                        == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 20 == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4
                        == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// Format specification for messages
                /// - more info: https://github.com/clemensv/spec/blob/registry-extensions/registry/spec.md#message-formats
                /// - message metadata can further decorate w/ things like format version, and so on.
                #[component(enum)]
                pub enum FormatSpec {
                    #[component(name = "cloudevents")]
                    Cloudevents,
                    #[component(name = "http")]
                    Http,
                    #[component(name = "amqp")]
                    Amqp,
                    #[component(name = "mqtt")]
                    Mqtt,
                    #[component(name = "kafka")]
                    Kafka,
                    #[component(name = "raw")]
                    Raw,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for FormatSpec {
                    #[inline]
                    fn clone(&self) -> FormatSpec {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for FormatSpec {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for FormatSpec {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for FormatSpec {
                    #[inline]
                    fn eq(&self, other: &FormatSpec) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for FormatSpec {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                unsafe impl wasmtime::component::Lower for FormatSpec {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut std::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::Cloudevents => {
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
                                                    m.map(|p| &raw mut (*p).Cloudevents)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Http => {
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
                                                    m.map(|p| &raw mut (*p).Http)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Amqp => {
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
                                                    m.map(|p| &raw mut (*p).Amqp)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Mqtt => {
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
                                                    m.map(|p| &raw mut (*p).Mqtt)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Kafka => {
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
                                                    m.map(|p| &raw mut (*p).Kafka)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Raw => {
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
                                                    m.map(|p| &raw mut (*p).Raw)
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
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
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
                            Self::Cloudevents => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Http => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Amqp => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Mqtt => {
                                *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Kafka => {
                                *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Raw => {
                                *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for FormatSpec {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::Cloudevents,
                                1u32 => Self::Http,
                                2u32 => Self::Amqp,
                                3u32 => Self::Mqtt,
                                4u32 => Self::Kafka,
                                5u32 => Self::Raw,
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
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::Cloudevents,
                                1u8 => Self::Http,
                                2u8 => Self::Amqp,
                                3u8 => Self::Mqtt,
                                4u8 => Self::Kafka,
                                5u8 => Self::Raw,
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
                    pub struct LowerFormatSpec {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadFormatSpec,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerFormatSpec {
                        #[inline]
                        fn clone(&self) -> LowerFormatSpec {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            let _: ::core::clone::AssertParamIsClone<
                                LowerPayloadFormatSpec,
                            >;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerFormatSpec {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadFormatSpec {
                        Cloudevents: [wasmtime::ValRaw; 0],
                        Http: [wasmtime::ValRaw; 0],
                        Amqp: [wasmtime::ValRaw; 0],
                        Mqtt: [wasmtime::ValRaw; 0],
                        Kafka: [wasmtime::ValRaw; 0],
                        Raw: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::clone::Clone for LowerPayloadFormatSpec {
                        #[inline]
                        fn clone(&self) -> LowerPayloadFormatSpec {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::marker::Copy for LowerPayloadFormatSpec {}
                    unsafe impl wasmtime::component::ComponentType for FormatSpec {
                        type Lower = LowerFormatSpec;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &["cloudevents", "http", "amqp", "mqtt", "kafka", "raw"],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[None, None, None, None, None, None],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for FormatSpec {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[None, None, None, None, None, None];
                    }
                };
                impl core::fmt::Debug for FormatSpec {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            FormatSpec::Cloudevents => {
                                f.debug_tuple("FormatSpec::Cloudevents").finish()
                            }
                            FormatSpec::Http => {
                                f.debug_tuple("FormatSpec::Http").finish()
                            }
                            FormatSpec::Amqp => {
                                f.debug_tuple("FormatSpec::Amqp").finish()
                            }
                            FormatSpec::Mqtt => {
                                f.debug_tuple("FormatSpec::Mqtt").finish()
                            }
                            FormatSpec::Kafka => {
                                f.debug_tuple("FormatSpec::Kafka").finish()
                            }
                            FormatSpec::Raw => f.debug_tuple("FormatSpec::Raw").finish(),
                        }
                    }
                }
                const _: () = {
                    if !(1 == <FormatSpec as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <FormatSpec as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1
                        == <FormatSpec as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <FormatSpec as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                /// A message with a binary payload, a format specification, and decorative metadata.
                #[component(record)]
                pub struct Message {
                    #[component(name = "data")]
                    pub data: Vec<u8>,
                    #[component(name = "format")]
                    pub format: FormatSpec,
                    #[component(name = "metadata")]
                    pub metadata: Option<Vec<(String, String)>>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Message {
                    #[inline]
                    fn clone(&self) -> Message {
                        Message {
                            data: ::core::clone::Clone::clone(&self.data),
                            format: ::core::clone::Clone::clone(&self.format),
                            metadata: ::core::clone::Clone::clone(&self.metadata),
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for Message {
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
                            &self.data,
                            cx,
                            ty.fields[0usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).data)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.format,
                            cx,
                            ty.fields[1usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).format)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.metadata,
                            cx,
                            ty.fields[2usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).metadata)
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
                            &self.data,
                            cx,
                            ty.fields[0usize].ty,
                            <Vec<u8> as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.format,
                            cx,
                            ty.fields[1usize].ty,
                            <FormatSpec as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.metadata,
                            cx,
                            ty.fields[2usize].ty,
                            <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for Message {
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
                            data: <Vec<
                                u8,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[0usize].ty,
                                &src.data,
                            )?,
                            format: <FormatSpec as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[1usize].ty,
                                &src.format,
                            )?,
                            metadata: <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[2usize].ty,
                                &src.metadata,
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
                            data: <Vec<
                                u8,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<Vec<u8> as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            format: <FormatSpec as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[1usize].ty,
                                &bytes[<FormatSpec as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<FormatSpec as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            metadata: <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[2usize].ty,
                                &bytes[<Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerMessage<T0: Copy, T1: Copy, T2: Copy> {
                        data: T0,
                        format: T1,
                        metadata: T2,
                        _align: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                        T2: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerMessage<T0, T1, T2> {
                        #[inline]
                        fn clone(&self) -> LowerMessage<T0, T1, T2> {
                            LowerMessage {
                                data: ::core::clone::Clone::clone(&self.data),
                                format: ::core::clone::Clone::clone(&self.format),
                                metadata: ::core::clone::Clone::clone(&self.metadata),
                                _align: ::core::clone::Clone::clone(&self._align),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                        T2: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy for LowerMessage<T0, T1, T2> {}
                    unsafe impl wasmtime::component::ComponentType for Message {
                        type Lower = LowerMessage<
                            <Vec<u8> as wasmtime::component::ComponentType>::Lower,
                            <FormatSpec as wasmtime::component::ComponentType>::Lower,
                            <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <Vec<u8> as wasmtime::component::ComponentType>::ABI,
                                <FormatSpec as wasmtime::component::ComponentType>::ABI,
                                <Option<
                                    Vec<(String, String)>,
                                > as wasmtime::component::ComponentType>::ABI,
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
                                        "data",
                                        <Vec<u8> as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "format",
                                        <FormatSpec as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "metadata",
                                        <Option<
                                            Vec<(String, String)>,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ],
                            )
                        }
                    }
                };
                impl core::fmt::Debug for Message {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("Message")
                            .field("data", &self.data)
                            .field("format", &self.format)
                            .field("metadata", &self.metadata)
                            .finish()
                    }
                }
                const _: () = {
                    if !(24 == <Message as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <Message as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Message as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Message as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub trait Host: HostClient + HostError {}
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                    U: Host + Send,
                {
                    let mut inst = linker
                        .instance("wasi:messaging/messaging-types@0.2.0-draft")?;
                    inst.resource(
                        "client",
                        wasmtime::component::ResourceType::host::<Client>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostClient::drop(
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
                    inst.func_wrap_async(
                        "[static]client.connect",
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
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
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
                                                                &"messaging-types" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[static]client.connect" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
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
                            let r = HostClient::connect(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
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
                        "[static]error.trace",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
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
                                                                &"messaging-types" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"[static]error.trace" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
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
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            let host = get(caller.data_mut());
                            let r = HostError::trace(host).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::messaging_types",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::messaging_types",
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
            pub mod producer {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Client = super::super::super::wasi::messaging::messaging_types::Client;
                pub type Channel = super::super::super::wasi::messaging::messaging_types::Channel;
                const _: () = {
                    if !(8 == <Channel as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <Channel as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Channel as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Channel as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub type Message = super::super::super::wasi::messaging::messaging_types::Message;
                const _: () = {
                    if !(24 == <Message as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <Message as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Message as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Message as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub type Error = super::super::super::wasi::messaging::messaging_types::Error;
                pub trait Host {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn send<'life0, 'async_trait>(
                        &'life0 mut self,
                        c: wasmtime::component::Resource<Client>,
                        ch: Channel,
                        m: Vec<Message>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<(), wasmtime::component::Resource<Error>>,
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
                    let mut inst = linker
                        .instance("wasi:messaging/producer@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "send",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (
                                wasmtime::component::Resource<Client>,
                                Channel,
                                Vec<Message>,
                            )|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::producer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::producer",
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
                                                            ::core::option::Option::Some(&"producer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"send" as &dyn Value),
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::producer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::producer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "c", "ch", "m"],
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
                            let r = Host::send(host, arg0, arg1, arg2).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::producer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::producer",
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
            pub mod consumer {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Client = super::super::super::wasi::messaging::messaging_types::Client;
                pub type Message = super::super::super::wasi::messaging::messaging_types::Message;
                const _: () = {
                    if !(24 == <Message as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 24 == <Message as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Message as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Message as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub type Channel = super::super::super::wasi::messaging::messaging_types::Channel;
                const _: () = {
                    if !(8 == <Channel as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 8 == <Channel as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <Channel as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <Channel as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub type Error = super::super::super::wasi::messaging::messaging_types::Error;
                pub type GuestConfiguration = super::super::super::wasi::messaging::messaging_types::GuestConfiguration;
                const _: () = {
                    if !(20
                        == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 20 == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4
                        == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub trait Host {
                    /// Blocking receive for t-milliseconds with ephemeral subscription – if no message is received, returns None
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn subscribe_try_receive<'life0, 'async_trait>(
                        &'life0 mut self,
                        c: wasmtime::component::Resource<Client>,
                        ch: Channel,
                        t_milliseconds: u32,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<
                                        Option<Vec<Message>>,
                                        wasmtime::component::Resource<Error>,
                                    >,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Blocking receive until message with ephemeral subscription
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn subscribe_receive<'life0, 'async_trait>(
                        &'life0 mut self,
                        c: wasmtime::component::Resource<Client>,
                        ch: Channel,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<Vec<Message>, wasmtime::component::Resource<Error>>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// 'Fit-all' type function for updating a guest's configuration – this could be useful for:
                    /// - unsubscribing from a channel,
                    /// - checkpointing,
                    /// - etc..
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn update_guest_configuration<'life0, 'async_trait>(
                        &'life0 mut self,
                        gc: GuestConfiguration,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<(), wasmtime::component::Resource<Error>>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// A message can exist under several statuses:
                    /// (1) available: the message is ready to be read,
                    /// (2) acquired: the message has been sent to a consumer (but still exists in the queue),
                    /// (3) accepted (result of complete-message): the message has been received and ACK-ed by a consumer and can be safely removed from the queue,
                    /// (4) rejected (result of abandon-message): the message has been received and NACK-ed by a consumer, at which point it can be:
                    /// - deleted,
                    /// - sent to a dead-letter queue, or
                    /// - kept in the queue for further processing.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn complete_message<'life0, 'async_trait>(
                        &'life0 mut self,
                        m: Message,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<(), wasmtime::component::Resource<Error>>,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn abandon_message<'life0, 'async_trait>(
                        &'life0 mut self,
                        m: Message,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::Result<
                                    Result<(), wasmtime::component::Resource<Error>>,
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
                    let mut inst = linker
                        .instance("wasi:messaging/consumer@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "subscribe-try-receive",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (wasmtime::component::Resource<Client>, Channel, u32)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                                                            ::core::option::Option::Some(&"consumer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"subscribe-try-receive" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "c", "ch", "t_milliseconds"],
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
                            let r = Host::subscribe_try_receive(host, arg0, arg1, arg2)
                                .await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                        "subscribe-receive",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Client>, Channel)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                                                            ::core::option::Option::Some(&"consumer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"subscribe-receive" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "c", "ch"],
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
                            let r = Host::subscribe_receive(host, arg0, arg1).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                        "update-guest-configuration",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (GuestConfiguration,)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                                                            ::core::option::Option::Some(&"consumer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"update-guest-configuration" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "gc"],
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
                            let r = Host::update_guest_configuration(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                        "complete-message",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (Message,)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                                                            ::core::option::Option::Some(&"consumer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"complete-message" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "m"],
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
                            let r = Host::complete_message(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                        "abandon-message",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (Message,)|
                        Box::new(async move {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
                                                            ::core::option::Option::Some(&"consumer" as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &"abandon-message" as &dyn Value,
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
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message", "m"],
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
                            let r = Host::abandon_message(host, arg0).await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/wasi-messaging/src/lib.rs:27",
                                            "wasi_messaging::bindings::wasi::messaging::consumer",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::wasi::messaging::consumer",
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
            pub mod messaging {
                #[allow(clippy::all)]
                pub mod messaging_guest {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    pub type Message = super::super::super::super::wasi::messaging::messaging_types::Message;
                    const _: () = {
                        if !(24
                            == <Message as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 24 == <Message as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <Message as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Message as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type GuestConfiguration = super::super::super::super::wasi::messaging::messaging_types::GuestConfiguration;
                    const _: () = {
                        if !(20
                            == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 20 == <GuestConfiguration as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <GuestConfiguration as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Error = super::super::super::super::wasi::messaging::messaging_types::Error;
                    pub struct Guest {
                        configure: wasmtime::component::Func,
                        handler: wasmtime::component::Func,
                    }
                    impl Guest {
                        pub fn new(
                            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                        ) -> wasmtime::Result<Guest> {
                            let configure = *__exports
                                .typed_func::<
                                    (),
                                    (
                                        Result<
                                            GuestConfiguration,
                                            wasmtime::component::Resource<Error>,
                                        >,
                                    ),
                                >("configure")?
                                .func();
                            let handler = *__exports
                                .typed_func::<
                                    (&[Message],),
                                    (Result<(), wasmtime::component::Resource<Error>>,),
                                >("handler")?
                                .func();
                            Ok(Guest { configure, handler })
                        }
                        /// Returns the list of channels (and extension metadata within guest-configuration) that
                        /// this component should subscribe to and be handled by the subsequent handler within guest-configuration
                        pub async fn call_configure<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<
                            Result<
                                GuestConfiguration,
                                wasmtime::component::Resource<Error>,
                            >,
                        >
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "wasi_messaging::bindings::exports::wasi::messaging::messaging_guest",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::exports::wasi::messaging::messaging_guest",
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
                                                                &"wasi:messaging/messaging-guest@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"configure" as &dyn Value),
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
                                    (),
                                    (
                                        Result<
                                            GuestConfiguration,
                                            wasmtime::component::Resource<Error>,
                                        >,
                                    ),
                                >::new_unchecked(self.configure)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Whenever this guest receives a message in one of the subscribed channels, the message is sent to this handler
                        pub async fn call_handler<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: &[Message],
                        ) -> wasmtime::Result<
                            Result<(), wasmtime::component::Resource<Error>>,
                        >
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "wasi_messaging::bindings::exports::wasi::messaging::messaging_guest",
                                            tracing::Level::TRACE,
                                            ::core::option::Option::Some(
                                                "crates/wasi-messaging/src/lib.rs",
                                            ),
                                            ::core::option::Option::Some(27u32),
                                            ::core::option::Option::Some(
                                                "wasi_messaging::bindings::exports::wasi::messaging::messaging_guest",
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
                                                                &"wasi:messaging/messaging-guest@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&"handler" as &dyn Value),
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
                                    (&[Message],),
                                    (Result<(), wasmtime::component::Resource<Error>>,),
                                >::new_unchecked(self.handler)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0,))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                    }
                }
            }
        }
    }
    const _: &str = "package wasi:messaging@0.2.0-draft;\n\nworld imports {\n\timport producer;\n\timport consumer;\n}\n\nworld messaging {\n\tinclude imports;\n\texport messaging-guest;\n}";
    const _: &str = "interface consumer {\n    // {client, message, channel, error, guest-configuration}\n    use messaging-types.{client, message, channel, error, guest-configuration};\n\n    /// Blocking receive for t-milliseconds with ephemeral subscription \u{2013}\u{a0}if no message is received, returns None\n    subscribe-try-receive: func(c: client, ch: channel, t-milliseconds: u32) -> result<option<list<message>>, error>;\n\n    /// Blocking receive until message with ephemeral subscription\n    subscribe-receive: func(c: client, ch: channel) -> result<list<message>, error>;\n\n    /// \'Fit-all\' type function for updating a guest\'s configuration \u{2013} this could be useful for:\n    ///     - unsubscribing from a channel,\n    ///     - checkpointing,\n    ///     - etc..\n    update-guest-configuration: func(gc: guest-configuration) -> result<_, error>;\n\n    /// A message can exist under several statuses:\n    /// (1) available: the message is ready to be read,\n    /// (2) acquired: the message has been sent to a consumer (but still exists in the queue),\n    /// (3) accepted (result of complete-message): the message has been received and ACK-ed by a consumer and can be safely removed from the queue,\n    /// (4) rejected (result of abandon-message): the message has been received and NACK-ed by a consumer, at which point it can be:\n    ///         - deleted,\n    ///         - sent to a dead-letter queue, or\n    ///         - kept in the queue for further processing.\n    complete-message: func(m: message) -> result<_, error>;\n    abandon-message: func(m: message) -> result<_, error>;\n}";
    const _: &str = "interface messaging-guest {\n    use messaging-types.{message, guest-configuration, error};\n\n    /// Returns the list of channels (and extension metadata within guest-configuration) that \n    /// this component should subscribe to and be handled by the subsequent handler within guest-configuration\n    configure: func() -> result<guest-configuration, error>;\n\n    /// Whenever this guest receives a message in one of the subscribed channels, the message is sent to this handler\n    handler: func(ms: list<message>) -> result<_, error>;\n}";
    const _: &str = "interface producer {\n    use messaging-types.{client, channel, message, error};\n    \n    send: func(c: client, ch: channel, m: list<message>) -> result<_, error>;\n}";
    const _: &str = "interface messaging-types {\n    /// A connection to a message-exchange service (e.g., buffer, broker, etc.).\n    resource client {\n        connect: static func(name: string) -> result<client, error>;\n    }\n    \n    /// TODO(danbugs): This should be eventually extracted as an underlying type for other wasi-cloud-core interfaces.\n    resource error {\n        trace: static func() -> string;    \n    }\n  \n    /// There are two types of channels:\n    /// - publish-subscribe channel, which is a broadcast channel, and\n    /// - point-to-point channel, which is a unicast channel.\n    ///\n    /// The interface doesn\'t highlight this difference in the type itself as that\'s uniquely a consumer issue.\n    type channel = string;\n  \n    /// Configuration includes a required list of channels the guest is subscribing to, and an optional list of extensions key-value pairs \n    /// (e.g., partitions/offsets to read from in Kafka/EventHubs, QoS etc.).\n    record guest-configuration {\n        channels: list<channel>,\n        extensions: option<list<tuple<string, string>>>\n    }\n  \n    /// Format specification for messages \n    ///  - more info: https://github.com/clemensv/spec/blob/registry-extensions/registry/spec.md#message-formats\n    ///  - message metadata can further decorate w/ things like format version, and so on.\n    enum format-spec {\n        cloudevents,\n        http,\n        amqp,\n        mqtt,\n        kafka,\n        raw\n    }\n  \n    /// A message with a binary payload, a format specification, and decorative metadata.\n    record message {\n        data: list<u8>,\n        format: format-spec,\n        metadata: option<list<tuple<string, string>>>\n    }\n}";
}
pub use bindings::exports;
/// MessageView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
pub trait MessagingView: WasiView + Send {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn connect<'life0, 'async_trait>(
        &'life0 mut self,
        name: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<Resource<Client>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn update_configuration<'life0, 'async_trait>(
        &'life0 mut self,
        gc: GuestConfiguration,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<(), Resource<Error>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
impl<T: MessagingView> messaging_types::Host for T {}
impl<T: MessagingView> HostClient for T {
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
    fn connect<'life0, 'async_trait>(
        &'life0 mut self,
        name: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = wasmtime::Result<
                    anyhow::Result<Resource<Client>, Resource<Error>>,
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
                wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let mut __self = self;
            let name = name;
            let __ret: wasmtime::Result<
                anyhow::Result<Resource<Client>, Resource<Error>>,
            > = { Ok(Ok(T::connect(__self, name).await?)) };
            #[allow(unreachable_code)] __ret
        })
    }
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        self.table().delete(client)?;
        Ok(())
    }
}
impl<T: MessagingView> HostError for T {
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
    fn trace<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = wasmtime::Result<String>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                wasmtime::Result<String>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let mut __self = self;
            let __ret: wasmtime::Result<String> = {
                Ok(String::from("TODO: trace HostError"))
            };
            #[allow(unreachable_code)] __ret
        })
    }
    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        {
            ::std::io::_print(format_args!("TODO: implement drop for {0:?}\n", err));
        };
        Ok(())
    }
}
/// RuntimeClient is implemented by the runtime to provide this host with access
/// to runtime functionality.
pub trait RuntimeClient: Sync + Send {
    /// Subscribe to the specified channel.
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn subscribe<'life0, 'async_trait>(
        &'life0 self,
        ch: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<Subscriber>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
    /// Publish a message to the specified channel.
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn publish<'life0, 'async_trait>(
        &'life0 self,
        ch: String,
        data: Bytes,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<()>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
/// RuntimeSubscriber is implemented by the runtime to provide the host with access
/// to runtime subscriber functionality.
pub trait RuntimeSubscriber: Stream<Item = Message> + Send {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn unsubscribe<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<()>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
