#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod messaging {
    mod consumer {
        use anyhow::anyhow;
        use futures::stream::StreamExt;
        use tokio::time::{sleep, Duration};
        use wasmtime::component::Resource;
        use crate::messaging::types::WasiMessagingView;
        use crate::wasi::messaging::consumer;
        use crate::wasi::messaging::messaging_types::{
            Client, Error, FormatSpec, GuestConfiguration, Message,
        };
        impl<T: WasiMessagingView> consumer::Host for T {
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
                        let mut subscriber = match client.subscribe(ch).await {
                            Ok(s) => s,
                            Err(e) => {
                                return Err(
                                    ::anyhow::__private::must_use({
                                        use ::anyhow::__private::kind::*;
                                        let error = match e {
                                            error => (&error).anyhow_kind().new(error),
                                        };
                                        error
                                    }),
                                );
                            }
                        };
                        let _result = tokio::spawn(async move {
                            let stream = subscriber
                                .by_ref()
                                .take_until(
                                    sleep(Duration::from_millis(u64::from(t_milliseconds))),
                                );
                            let messages = stream
                                .map(|m| Message {
                                    data: m.payload.to_vec(),
                                    metadata: Some(
                                        <[_]>::into_vec(
                                            #[rustc_box]
                                            ::alloc::boxed::Box::new([
                                                (String::from("channel"), m.subject.to_string()),
                                            ]),
                                        ),
                                    ),
                                    format: FormatSpec::Raw,
                                })
                                .collect::<Vec<_>>()
                                .await;
                            let _ = subscriber.unsubscribe().await;
                            Ok::<Vec<Message>, Error>(messages)
                        });
                        Ok(Ok(None))
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
                        let mut subscriber = match client.subscribe(ch).await {
                            Ok(s) => s,
                            Err(e) => {
                                return Err(
                                    ::anyhow::__private::must_use({
                                        use ::anyhow::__private::kind::*;
                                        let error = match e {
                                            error => (&error).anyhow_kind().new(error),
                                        };
                                        error
                                    }),
                                );
                            }
                        };
                        let messages = subscriber
                            .by_ref()
                            .take(1)
                            .map(|m| Message {
                                data: m.payload.to_vec(),
                                metadata: Some(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            (String::from("channel"), m.subject.to_string()),
                                        ]),
                                    ),
                                ),
                                format: FormatSpec::Raw,
                            })
                            .collect::<Vec<_>>()
                            .await;
                        let _ = subscriber.unsubscribe().await;
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
                _gc: GuestConfiguration,
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
                    let _gc = _gc;
                    let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
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
            fn complete_message<'life0, 'async_trait>(
                &'life0 mut self,
                _msg: Message,
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
                    let _msg = _msg;
                    let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                        {
                            ::std::io::_print(
                                format_args!("Implement complete_message\n"),
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
                _msg: Message,
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
                    let _msg = _msg;
                    let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                        {
                            ::std::io::_print(
                                format_args!("Implement abandon_message\n"),
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
        use crate::messaging::types::WasiMessagingView;
        use crate::wasi::messaging::messaging_types::{Client, Error, Message};
        use crate::wasi::messaging::producer;
        impl<T: WasiMessagingView> producer::Host for T {
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
                msg: Vec<Message>,
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
                    let msg = msg;
                    let __ret: wasmtime::Result<anyhow::Result<(), Resource<Error>>> = {
                        {
                            ::std::io::_print(format_args!("send: ch: {0}\n", ch));
                        };
                        let data = Bytes::from(msg[0].data.clone());
                        let client = __self.table().get(&client)?;
                        client.publish(ch, data).await?;
                        Ok(Ok(()))
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod types {
        use bytes::Bytes;
        use wasmtime::component::Resource;
        use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};
        pub trait WasiMessagingView: WasiView + Send {
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
        }
    }
    use crate::wasi::messaging::Client;
    use wasmtime::component::Resource;
    pub use crate::messaging::types::WasiMessagingView;
    use crate::wasi::messaging::messaging_types::{self, Error, HostClient, HostError};
    impl<T: WasiMessagingView> messaging_types::Host for T {}
    impl<T: WasiMessagingView> HostClient for T {
        /// Connect to the NATS server specified by `name` and return a client resource.
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
                > = {
                    let resource = __self.connect(name).await?;
                    Ok(Ok(resource))
                };
                #[allow(unreachable_code)] __ret
            })
        }
        /// Drop the specified NATS client resource.
        fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
            let _ = self.table().delete(client)?;
            Ok(())
        }
    }
    impl<T: WasiMessagingView> HostError for T {
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
                    Ok(String::from("trace HostError"))
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
            {
                ::std::io::_print(format_args!("Implement drop for {0:?}\n", err));
            };
            Ok(())
        }
    }
}
mod nats {
    use std::collections::HashMap;
    use futures::stream::{self, StreamExt};
    use wasmtime::component::{Component, Linker, Resource};
    use wasmtime::{Engine, Store};
    use wasmtime_wasi::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
    use crate::messaging;
    use crate::wasi::messaging::messaging_types::{FormatSpec, Message};
    /// Host is the base type used to implement host messaging interfaces.
    /// In addition, it holds the "host-defined state" used by the wasm runtime [`Store`].
    pub struct Host {
        keys: HashMap<String, u32>,
        pub table: ResourceTable,
        ctx: WasiCtx,
    }
    impl Host {
        pub fn new() -> Self {
            Self {
                keys: HashMap::default(),
                table: ResourceTable::default(),
                ctx: WasiCtxBuilder::new().inherit_env().build(),
            }
        }
    }
    impl messaging::WasiMessagingView for Host {
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
                    Output = anyhow::Result<Resource<Client>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    anyhow::Result<Resource<Client>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let name = name;
                let __ret: anyhow::Result<Resource<Client>> = {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("connect")),
                        );
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn ctx(&mut self) -> &mut WasiCtx {
            {
                ::core::panicking::panic_fmt(
                    format_args!("not implemented: {0}", format_args!("ctx")),
                );
            }
        }
        fn table(&mut self) -> &mut ResourceTable {
            {
                ::core::panicking::panic_fmt(
                    format_args!("not implemented: {0}", format_args!("table")),
                );
            }
        }
    }
    impl WasiView for Host {
        fn ctx(&mut self) -> &mut WasiCtx {
            {
                ::core::panicking::panic_fmt(
                    format_args!("not implemented: {0}", format_args!("ctx")),
                );
            }
        }
        fn table(&mut self) -> &mut ResourceTable {
            {
                ::core::panicking::panic_fmt(
                    format_args!("not implemented: {0}", format_args!("table")),
                );
            }
        }
    }
    pub struct Client {
        pub inner: async_nats::Client,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Client {
        #[inline]
        fn clone(&self) -> Client {
            Client {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    impl Client {
        pub async fn subscribe(
            &self,
            ch: String,
        ) -> anyhow::Result<async_nats::Subscriber> {
            Ok(self.inner.subscribe(ch).await?)
        }
        pub async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
            Ok(self.inner.publish(ch, data).await?)
        }
    }
    pub async fn serve(engine: &Engine, wasm: String) -> anyhow::Result<()> {
        let mut store = Store::new(engine, messaging::Host::new());
        let component = Component::from_file(engine, wasm)?;
        let mut linker = Linker::new(engine);
        command::add_to_linker(&mut linker)?;
        crate::Messaging::add_to_linker(&mut linker, |t| t)?;
        let instance_pre = linker.instantiate_pre(&component)?;
        let (messaging, _) = crate::Messaging::instantiate_pre(&mut store, &instance_pre)
            .await?;
        let guest = messaging.wasi_messaging_messaging_guest();
        let host = store.data_mut();
        let Ok(client) = host.connect("demo.nats.io".to_string()).await? else {
            return Err(
                ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Failed to connect to NATS server"),
                    );
                    error
                }),
            );
        };
        let client = host.table.get(&client)?.clone();
        let Ok(gc) = guest.call_configure(&mut store).await? else {
            return Err(
                ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Failed to configure NATS client"),
                    );
                    error
                }),
            );
        };
        let mut subscribers = ::alloc::vec::Vec::new();
        for ch in &gc.channels {
            let subscriber = client.subscribe(ch.to_owned()).await?;
            subscribers.push(subscriber);
        }
        let mut messages = stream::select_all(subscribers);
        while let Some(message) = messages.next().await {
            let msg = Message {
                data: message.payload.to_vec(),
                metadata: Some(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            (String::from("channel"), message.subject.to_string()),
                        ]),
                    ),
                ),
                format: FormatSpec::Raw,
            };
            let _ = guest.call_handler(&mut store, &[msg]).await?;
        }
        Ok(())
    }
}
use anyhow::Error;
use clap::Parser;
pub use nats::Client;
use wasmtime::component::bindgen;
use wasmtime::{Config, Engine};
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
            U: wasi::messaging::messaging_types::Host + wasi::messaging::producer::Host
                + wasi::messaging::consumer::Host + Send,
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
            pub enum Error {}
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
                unsafe impl wasmtime::component::ComponentType for GuestConfiguration {
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
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
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
                        let _: ::core::clone::AssertParamIsClone<LowerPayloadFormatSpec>;
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
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        FormatSpec::Cloudevents => {
                            f.debug_tuple("FormatSpec::Cloudevents").finish()
                        }
                        FormatSpec::Http => f.debug_tuple("FormatSpec::Http").finish(),
                        FormatSpec::Amqp => f.debug_tuple("FormatSpec::Amqp").finish(),
                        FormatSpec::Mqtt => f.debug_tuple("FormatSpec::Mqtt").finish(),
                        FormatSpec::Kafka => f.debug_tuple("FormatSpec::Kafka").finish(),
                        FormatSpec::Raw => f.debug_tuple("FormatSpec::Raw").finish(),
                    }
                }
            }
            const _: () = {
                if !(1 == <FormatSpec as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <FormatSpec as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <FormatSpec as wasmtime::component::ComponentType>::ALIGN32) {
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
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = HostClient::connect(host, arg0).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = HostError::trace(host).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::messaging_types",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::messaging_types",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                let mut inst = linker.instance("wasi:messaging/producer@0.2.0-draft")?;
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
                                        "host::wasi::messaging::producer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::producer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::producer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::producer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = Host::send(host, arg0, arg1, arg2).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::producer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::producer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                let mut inst = linker.instance("wasi:messaging/consumer@0.2.0-draft")?;
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
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        Ok((r?,))
                    }),
                )?;
                inst.func_wrap_async(
                    "subscribe-receive",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Client>, Channel)|
                    Box::new(async move {
                        let span = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "wit-bindgen import",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = Host::subscribe_receive(host, arg0, arg1).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = Host::update_guest_configuration(host, arg0).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = Host::complete_message(host, arg0).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let _enter = span.enter();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        let host = get(caller.data_mut());
                        let r = Host::abandon_message(host, arg0).await;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:10",
                                        "host::wasi::messaging::consumer",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::wasi::messaging::consumer",
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
                                    if match tracing::Level::TRACE {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match tracing::Level::TRACE {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match tracing::Level::TRACE {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
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
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                        Result<GuestConfiguration, wasmtime::component::Resource<Error>>,
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
                                        "host::exports::wasi::messaging::messaging_guest",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::exports::wasi::messaging::messaging_guest",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
                                        "host::exports::wasi::messaging::messaging_guest",
                                        tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(10u32),
                                        ::core::option::Option::Some(
                                            "host::exports::wasi::messaging::messaging_guest",
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
                                if match tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{
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
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
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
/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    #[arg(short, long)]
    wasm: String,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for Args {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for Args {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("host");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("host");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for Args {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Args {
            wasm: __clap_arg_matches
                .remove_one::<String>("wasm")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: wasm",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("wasm") {
            #[allow(non_snake_case)]
            let wasm = &mut self.wasm;
            *wasm = __clap_arg_matches
                .remove_one::<String>("wasm")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: wasm",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for Args {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Args"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Args")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("wasm")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("wasm")
                        .value_name("WASM")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("The path to the wasm file to serve")
                        .long_help(None)
                        .short('w')
                        .long("wasm");
                    let arg = arg;
                    arg
                });
            __clap_app
                .about(
                    "Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable Credential wallet",
                )
                .long_about(None)
                .version("0.1.0")
                .about(
                    "host for WASM guest for secure, isolated and distributed storage of wallet keys and credentials",
                )
                .long_about(None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Args")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("wasm")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("wasm")
                        .value_name("WASM")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("The path to the wasm file to serve")
                        .long_help(None)
                        .short('w')
                        .long("wasm");
                    let arg = arg.required(false);
                    arg
                });
            __clap_app
                .about(
                    "Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable Credential wallet",
                )
                .long_about(None)
                .version("0.1.0")
                .about(
                    "host for WASM guest for secure, isolated and distributed storage of wallet keys and credentials",
                )
                .long_about(None)
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Args {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "Args",
            "wasm",
            &&self.wasm,
        )
    }
}
pub fn main() -> wasmtime::Result<()> {
    let body = async {
        let args = Args::parse();
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        tokio::spawn(async move { nats::serve(&engine, args.wasm).await });
        shutdown().await
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
async fn shutdown() -> Result<(), Error> {
    {
        #[doc(hidden)]
        mod __tokio_select_util {
            pub(super) enum Out<_0> {
                _0(_0),
                Disabled,
            }
            pub(super) type Mask = u8;
        }
        use ::tokio::macros::support::Future;
        use ::tokio::macros::support::Pin;
        use ::tokio::macros::support::Poll::{Ready, Pending};
        const BRANCHES: u32 = 1;
        let mut disabled: __tokio_select_util::Mask = Default::default();
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 0;
            disabled |= mask;
        }
        let mut output = {
            let mut futures = (tokio::signal::ctrl_c(),);
            let mut futures = &mut futures;
            ::tokio::macros::support::poll_fn(|cx| {
                    let mut is_pending = false;
                    let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                    for i in 0..BRANCHES {
                        let branch;
                        #[allow(clippy::modulo_one)]
                        {
                            branch = (start + i) % BRANCHES;
                        }
                        match branch {
                            #[allow(unreachable_code)]
                            0 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    _ => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_0(out));
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!(
                                            "reaching this means there probably is an off by one bug",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                    if is_pending {
                        Pending
                    } else {
                        Ready(__tokio_select_util::Out::Disabled)
                    }
                })
                .await
        };
        match output {
            __tokio_select_util::Out::_0(_) => Ok(()),
            __tokio_select_util::Out::Disabled => {
                ::core::panicking::panic_fmt(
                    format_args!("all branches are disabled and there is no else branch"),
                );
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("failed to match bind"),
                    ),
                );
            }
        }
    }
}
