/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use super::{Bucket, Error};
    type _TrappableError0 = Error;
    #[doc(hidden)]
    pub use Bucket as __with_name0;
    /// Auto-generated bindings for a pre-instantiated version of a
    /// component which implements the world `keyvalue`.
    ///
    /// This structure is created through [`KeyvaluePre::new`] which
    /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
    /// has been created through a [`Linker`](wasmtime::component::Linker).
    ///
    /// For more information see [`Keyvalue`] as well.
    pub struct KeyvaluePre<T> {
        instance_pre: wasmtime::component::InstancePre<T>,
        indices: KeyvalueIndices,
    }
    impl<T> Clone for KeyvaluePre<T> {
        fn clone(&self) -> Self {
            Self {
                instance_pre: self.instance_pre.clone(),
                indices: self.indices.clone(),
            }
        }
    }
    impl<_T> KeyvaluePre<_T> {
        /// Creates a new copy of `KeyvaluePre` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component behind `instance_pre`
        /// does not have the required exports.
        pub fn new(
            instance_pre: wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let indices = KeyvalueIndices::new(instance_pre.component())?;
            Ok(Self { instance_pre, indices })
        }
        pub fn engine(&self) -> &wasmtime::Engine {
            self.instance_pre.engine()
        }
        pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
            &self.instance_pre
        }
        /// Instantiates a new instance of [`Keyvalue`] within the
        /// `store` provided.
        ///
        /// This function will use `self` as the pre-instantiated
        /// instance to perform instantiation. Afterwards the preloaded
        /// indices in `self` are used to lookup all exports on the
        /// resulting instance.
        pub async fn instantiate_async(
            &self,
            mut store: impl wasmtime::AsContextMut<Data = _T>,
        ) -> wasmtime::Result<Keyvalue>
        where
            _T: Send,
        {
            let mut store = store.as_context_mut();
            let instance = self.instance_pre.instantiate_async(&mut store).await?;
            self.indices.load(&mut store, &instance)
        }
    }
    /// Auto-generated bindings for index of the exports of
    /// `keyvalue`.
    ///
    /// This is an implementation detail of [`KeyvaluePre`] and can
    /// be constructed if needed as well.
    ///
    /// For more information see [`Keyvalue`] as well.
    pub struct KeyvalueIndices {
        interface0: exports::wasi::keyvalue::watcher::GuestIndices,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeyvalueIndices {
        #[inline]
        fn clone(&self) -> KeyvalueIndices {
            KeyvalueIndices {
                interface0: ::core::clone::Clone::clone(&self.interface0),
            }
        }
    }
    /// Auto-generated bindings for an instance a component which
    /// implements the world `keyvalue`.
    ///
    /// This structure can be created through a number of means
    /// depending on your requirements and what you have on hand:
    ///
    /// * The most convenient way is to use
    ///   [`Keyvalue::instantiate_async`] which only needs a
    ///   [`Store`], [`Component`], and [`Linker`].
    ///
    /// * Alternatively you can create a [`KeyvaluePre`] ahead of
    ///   time with a [`Component`] to front-load string lookups
    ///   of exports once instead of per-instantiation. This
    ///   method then uses [`KeyvaluePre::instantiate_async`] to
    ///   create a [`Keyvalue`].
    ///
    /// * If you've instantiated the instance yourself already
    ///   then you can use [`Keyvalue::new`].
    ///
    /// * You can also access the guts of instantiation through
    ///   [`KeyvalueIndices::new_instance`] followed
    ///   by [`KeyvalueIndices::load`] to crate an instance of this
    ///   type.
    ///
    /// These methods are all equivalent to one another and move
    /// around the tradeoff of what work is performed when.
    ///
    /// [`Store`]: wasmtime::Store
    /// [`Component`]: wasmtime::component::Component
    /// [`Linker`]: wasmtime::component::Linker
    pub struct Keyvalue {
        interface0: exports::wasi::keyvalue::watcher::Guest,
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl KeyvalueIndices {
            /// Creates a new copy of `KeyvalueIndices` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component does not have the
            /// required exports.
            pub fn new(
                component: &wasmtime::component::Component,
            ) -> wasmtime::Result<Self> {
                let _component = component;
                let interface0 = exports::wasi::keyvalue::watcher::GuestIndices::new(
                    _component,
                )?;
                Ok(KeyvalueIndices { interface0 })
            }
            /// Creates a new instance of [`KeyvalueIndices`] from an
            /// instantiated component.
            ///
            /// This method of creating a [`Keyvalue`] will perform string
            /// lookups for all exports when this method is called. This
            /// will only succeed if the provided instance matches the
            /// requirements of [`Keyvalue`].
            pub fn new_instance(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Self> {
                let _instance = instance;
                let interface0 = exports::wasi::keyvalue::watcher::GuestIndices::new_instance(
                    &mut store,
                    _instance,
                )?;
                Ok(KeyvalueIndices { interface0 })
            }
            /// Uses the indices stored in `self` to load an instance
            /// of [`Keyvalue`] from the instance provided.
            ///
            /// Note that at this time this method will additionally
            /// perform type-checks of all exports.
            pub fn load(
                &self,
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Keyvalue> {
                let _instance = instance;
                let interface0 = self.interface0.load(&mut store, &_instance)?;
                Ok(Keyvalue { interface0 })
            }
        }
        impl Keyvalue {
            /// Convenience wrapper around [`KeyvaluePre::new`] and
            /// [`KeyvaluePre::instantiate_async`].
            pub async fn instantiate_async<_T>(
                mut store: impl wasmtime::AsContextMut<Data = _T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<_T>,
            ) -> wasmtime::Result<Keyvalue>
            where
                _T: Send,
            {
                let pre = linker.instantiate_pre(component)?;
                KeyvaluePre::new(pre)?.instantiate_async(store).await
            }
            /// Convenience wrapper around [`KeyvalueIndices::new_instance`] and
            /// [`KeyvalueIndices::load`].
            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Keyvalue> {
                let indices = KeyvalueIndices::new_instance(&mut store, instance)?;
                indices.load(store, instance)
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                T: Send,
                U: wasi::keyvalue::store::Host + wasi::keyvalue::atomics::Host
                    + wasi::keyvalue::batch::Host + Send,
            {
                wasi::keyvalue::store::add_to_linker(linker, get)?;
                wasi::keyvalue::atomics::add_to_linker(linker, get)?;
                wasi::keyvalue::batch::add_to_linker(linker, get)?;
                Ok(())
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
                use wasmtime::component::__internal::{anyhow, Box};
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
                    Other(wasmtime::component::__internal::String),
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
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
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
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
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
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
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
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
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
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
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
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
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
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
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
                                        <wasmtime::component::__internal::String as wasmtime::component::Lift>::lift(
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
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                )
                                            }),
                                        ),
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
                                        <wasmtime::component::__internal::String as wasmtime::component::Lift>::load(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<wasmtime::component::__internal::String as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                )
                                            }),
                                        ),
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
                            <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
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
                                            <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                ],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[
                                None,
                                None,
                                Some(
                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                ),
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
                            Some(
                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                            ),
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
                impl core::error::Error for Error {}
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
                    pub keys: wasmtime::component::__internal::Vec<
                        wasmtime::component::__internal::String,
                    >,
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
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
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
                                        use ::wasmtime::MaybeUninitExt;
                                        let m: &mut core::mem::MaybeUninit<_> = dst;
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
                                        use ::wasmtime::MaybeUninitExt;
                                        let m: &mut core::mem::MaybeUninit<_> = dst;
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
                            <wasmtime::component::__internal::Vec<
                                wasmtime::component::__internal::String,
                            > as wasmtime::component::ComponentType>::ABI
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
                            keys: <wasmtime::component::__internal::Vec<
                                wasmtime::component::__internal::String,
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
                            keys: <wasmtime::component::__internal::Vec<
                                wasmtime::component::__internal::String,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
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
                            <wasmtime::component::__internal::Vec<
                                wasmtime::component::__internal::String,
                            > as wasmtime::component::ComponentType>::Lower,
                            <Option<u64> as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                > as wasmtime::component::ComponentType>::ABI,
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
                                        <wasmtime::component::__internal::Vec<
                                            wasmtime::component::__internal::String,
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
                pub use super::super::super::__with_name0 as Bucket;
                pub trait HostBucket: Sized + ::core::marker::Send {
                    /// Get the value associated with the specified `key`
                    ///
                    /// The value is returned as an option. If the key-value pair exists in the
                    /// store, it returns `Ok(value)`. If the key does not exist in the
                    /// store, it returns `Ok(none)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn get(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            Option<wasmtime::component::__internal::Vec<u8>>,
                            super::super::super::_TrappableError0,
                        >,
                    > + ::core::marker::Send;
                    /// Set the value associated with the key in the store. If the key already
                    /// exists in the store, it overwrites the value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn set(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                        value: wasmtime::component::__internal::Vec<u8>,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
                    /// Delete the key-value pair associated with the key in the store.
                    ///
                    /// If the key does not exist in the store, it does nothing.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn delete(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
                    /// Check if the key exists in the store.
                    ///
                    /// If the key exists in the store, it returns `Ok(true)`. If the key does
                    /// not exist in the store, it returns `Ok(false)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn exists(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<bool, super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
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
                    fn list_keys(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        cursor: Option<u64>,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            KeyResponse,
                            super::super::super::_TrappableError0,
                        >,
                    > + ::core::marker::Send;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Bucket>,
                    ) -> impl ::core::future::Future<
                        Output = wasmtime::Result<()>,
                    > + ::core::marker::Send;
                }
                impl<_T: HostBucket + ?Sized + Send> HostBucket for &mut _T {
                    /// Get the value associated with the specified `key`
                    ///
                    /// The value is returned as an option. If the key-value pair exists in the
                    /// store, it returns `Ok(value)`. If the key does not exist in the
                    /// store, it returns `Ok(none)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn get(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> Result<
                        Option<wasmtime::component::__internal::Vec<u8>>,
                        super::super::super::_TrappableError0,
                    > {
                        HostBucket::get(*self, self_, key).await
                    }
                    /// Set the value associated with the key in the store. If the key already
                    /// exists in the store, it overwrites the value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn set(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                        value: wasmtime::component::__internal::Vec<u8>,
                    ) -> Result<(), super::super::super::_TrappableError0> {
                        HostBucket::set(*self, self_, key, value).await
                    }
                    /// Delete the key-value pair associated with the key in the store.
                    ///
                    /// If the key does not exist in the store, it does nothing.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn delete(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> Result<(), super::super::super::_TrappableError0> {
                        HostBucket::delete(*self, self_, key).await
                    }
                    /// Check if the key exists in the store.
                    ///
                    /// If the key exists in the store, it returns `Ok(true)`. If the key does
                    /// not exist in the store, it returns `Ok(false)`.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn exists(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                    ) -> Result<bool, super::super::super::_TrappableError0> {
                        HostBucket::exists(*self, self_, key).await
                    }
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
                    async fn list_keys(
                        &mut self,
                        self_: wasmtime::component::Resource<Bucket>,
                        cursor: Option<u64>,
                    ) -> Result<KeyResponse, super::super::super::_TrappableError0> {
                        HostBucket::list_keys(*self, self_, cursor).await
                    }
                    async fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Bucket>,
                    ) -> wasmtime::Result<()> {
                        HostBucket::drop(*self, rep).await
                    }
                }
                pub trait Host: Send + HostBucket + Sized + ::core::marker::Send {
                    /// Get the bucket with the specified identifier.
                    ///
                    /// `identifier` must refer to a bucket provided by the host.
                    ///
                    /// `error::no-such-store` will be raised if the `identifier` is not recognized.
                    fn open(
                        &mut self,
                        identifier: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::Resource<Bucket>,
                            super::super::super::_TrappableError0,
                        >,
                    > + ::core::marker::Send;
                    fn convert_error(
                        &mut self,
                        err: super::super::super::_TrappableError0,
                    ) -> wasmtime::Result<Error>;
                }
                pub trait GetHost<
                    T,
                    D,
                >: Fn(
                        T,
                    ) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static {
                    type Host: Host + Send;
                }
                impl<F, T, D, O> GetHost<T, D> for F
                where
                    F: Fn(T) -> O + Send + Sync + Copy + 'static,
                    O: Host + Send,
                {
                    type Host = O;
                }
                pub fn add_to_linker_get_host<
                    T,
                    G: for<'a> GetHost<&'a mut T, T, Host: Host + Send>,
                >(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: G,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                {
                    let mut inst = linker.instance("wasi:keyvalue/store@0.2.0-draft")?;
                    inst.resource_async(
                        "bucket",
                        wasmtime::component::ResourceType::host::<Bucket>(),
                        move |mut store, rep| {
                            wasmtime::component::__internal::Box::new(async move {
                                HostBucket::drop(
                                        &mut host_getter(store.data_mut()),
                                        wasmtime::component::Resource::new_own(rep),
                                    )
                                    .await
                            })
                        },
                    )?;
                    inst.func_wrap_async(
                        "open",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::__internal::String,)|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"open" as &dyn Value,
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::open(host, arg0).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]bucket.get",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::String,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostBucket::get(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]bucket.set",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::String,
                                wasmtime::component::__internal::Vec<u8>,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostBucket::set(host, arg0, arg1, arg2).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]bucket.delete",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::String,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostBucket::delete(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]bucket.exists",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::String,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostBucket::exists(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]bucket.list-keys",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Bucket>, Option<u64>)|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::store",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"store" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostBucket::list_keys(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::store",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::store",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => Err(Host::convert_error(host, e)?),
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    U: Host + Send,
                    T: Send,
                {
                    add_to_linker_get_host(linker, get)
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    /// Get the bucket with the specified identifier.
                    ///
                    /// `identifier` must refer to a bucket provided by the host.
                    ///
                    /// `error::no-such-store` will be raised if the `identifier` is not recognized.
                    async fn open(
                        &mut self,
                        identifier: wasmtime::component::__internal::String,
                    ) -> Result<
                        wasmtime::component::Resource<Bucket>,
                        super::super::super::_TrappableError0,
                    > {
                        Host::open(*self, identifier).await
                    }
                    fn convert_error(
                        &mut self,
                        err: super::super::super::_TrappableError0,
                    ) -> wasmtime::Result<Error> {
                        Host::convert_error(*self, err)
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod atomics {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
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
                pub trait Host: Send + ::core::marker::Send {
                    /// Atomically increment the value associated with the key in the store by the given delta. It
                    /// returns the new value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair with the value set
                    /// to the given delta.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn increment(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                        delta: u64,
                    ) -> impl ::core::future::Future<
                        Output = Result<u64, super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
                }
                pub trait GetHost<
                    T,
                    D,
                >: Fn(
                        T,
                    ) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static {
                    type Host: Host
                        + Send
                        + super::super::super::wasi::keyvalue::store::Host;
                }
                impl<F, T, D, O> GetHost<T, D> for F
                where
                    F: Fn(T) -> O + Send + Sync + Copy + 'static,
                    O: Host + Send + super::super::super::wasi::keyvalue::store::Host,
                {
                    type Host = O;
                }
                pub fn add_to_linker_get_host<
                    T,
                    G: for<'a> GetHost<
                            &'a mut T,
                            T,
                            Host: Host + Send
                                + super::super::super::wasi::keyvalue::store::Host,
                        >,
                >(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: G,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
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
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::String,
                                u64,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::atomics",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::atomics",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"atomics" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"increment" as &dyn Value,
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::atomics",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::atomics",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg1) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg2) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::increment(host, arg0, arg1, arg2).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::atomics",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::atomics",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => {
                                                Err(
                                                    super::super::super::wasi::keyvalue::store::Host::convert_error(
                                                        host,
                                                        e,
                                                    )?,
                                                )
                                            }
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    U: Host + Send + super::super::super::wasi::keyvalue::store::Host,
                    T: Send,
                {
                    add_to_linker_get_host(linker, get)
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    /// Atomically increment the value associated with the key in the store by the given delta. It
                    /// returns the new value.
                    ///
                    /// If the key does not exist in the store, it creates a new key-value pair with the value set
                    /// to the given delta.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn increment(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key: wasmtime::component::__internal::String,
                        delta: u64,
                    ) -> Result<u64, super::super::super::_TrappableError0> {
                        Host::increment(*self, bucket, key, delta).await
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod batch {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
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
                pub trait Host: Send + ::core::marker::Send {
                    /// Get the key-value pairs associated with the keys in the store. It returns a list of
                    /// key-value pairs.
                    ///
                    /// If any of the keys do not exist in the store, it returns a `none` value for that pair in the
                    /// list.
                    ///
                    /// MAY show an out-of-date value if there are concurrent writes to the store.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    fn get_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::__internal::Vec<
                                Option<
                                    (
                                        wasmtime::component::__internal::String,
                                        wasmtime::component::__internal::Vec<u8>,
                                    ),
                                >,
                            >,
                            super::super::super::_TrappableError0,
                        >,
                    > + ::core::marker::Send;
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
                    fn set_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key_values: wasmtime::component::__internal::Vec<
                            (
                                wasmtime::component::__internal::String,
                                wasmtime::component::__internal::Vec<u8>,
                            ),
                        >,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
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
                    fn delete_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + ::core::marker::Send;
                }
                pub trait GetHost<
                    T,
                    D,
                >: Fn(
                        T,
                    ) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static {
                    type Host: Host
                        + Send
                        + super::super::super::wasi::keyvalue::store::Host;
                }
                impl<F, T, D, O> GetHost<T, D> for F
                where
                    F: Fn(T) -> O + Send + Sync + Copy + 'static,
                    O: Host + Send + super::super::super::wasi::keyvalue::store::Host,
                {
                    type Host = O;
                }
                pub fn add_to_linker_get_host<
                    T,
                    G: for<'a> GetHost<
                            &'a mut T,
                            T,
                            Host: Host + Send
                                + super::super::super::wasi::keyvalue::store::Host,
                        >,
                >(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: G,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                {
                    let mut inst = linker.instance("wasi:keyvalue/batch@0.2.0-draft")?;
                    inst.func_wrap_async(
                        "get-many",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"batch" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"get-many" as &dyn Value,
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::get_many(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => {
                                                Err(
                                                    super::super::super::wasi::keyvalue::store::Host::convert_error(
                                                        host,
                                                        e,
                                                    )?,
                                                )
                                            }
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
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
                                wasmtime::component::__internal::Vec<
                                    (
                                        wasmtime::component::__internal::String,
                                        wasmtime::component::__internal::Vec<u8>,
                                    ),
                                >,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"batch" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"set-many" as &dyn Value,
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::set_many(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => {
                                                Err(
                                                    super::super::super::wasi::keyvalue::store::Host::convert_error(
                                                        host,
                                                        e,
                                                    )?,
                                                )
                                            }
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "delete-many",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Bucket>,
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                            )|
                        {
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen import",
                                            "services::keyvalue::generated::wasi::keyvalue::batch",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"batch" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"delete-many" as &dyn Value,
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
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("call") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&arg0) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug("...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::delete_many(host, arg0, arg1).await;
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/services/src/keyvalue.rs:13",
                                                    "services::keyvalue::generated::wasi::keyvalue::batch",
                                                    tracing::Level::TRACE,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/services/src/keyvalue.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(13u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "services::keyvalue::generated::wasi::keyvalue::batch",
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
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("return") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &tracing::field::debug(&r) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Ok((
                                        match r {
                                            Ok(a) => Ok(a),
                                            Err(e) => {
                                                Err(
                                                    super::super::super::wasi::keyvalue::store::Host::convert_error(
                                                        host,
                                                        e,
                                                    )?,
                                                )
                                            }
                                        },
                                    ))
                                }
                                    .instrument(span),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    U: Host + Send + super::super::super::wasi::keyvalue::store::Host,
                    T: Send,
                {
                    add_to_linker_get_host(linker, get)
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    /// Get the key-value pairs associated with the keys in the store. It returns a list of
                    /// key-value pairs.
                    ///
                    /// If any of the keys do not exist in the store, it returns a `none` value for that pair in the
                    /// list.
                    ///
                    /// MAY show an out-of-date value if there are concurrent writes to the store.
                    ///
                    /// If any other error occurs, it returns an `Err(error)`.
                    async fn get_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> Result<
                        wasmtime::component::__internal::Vec<
                            Option<
                                (
                                    wasmtime::component::__internal::String,
                                    wasmtime::component::__internal::Vec<u8>,
                                ),
                            >,
                        >,
                        super::super::super::_TrappableError0,
                    > {
                        Host::get_many(*self, bucket, keys).await
                    }
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
                    async fn set_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        key_values: wasmtime::component::__internal::Vec<
                            (
                                wasmtime::component::__internal::String,
                                wasmtime::component::__internal::Vec<u8>,
                            ),
                        >,
                    ) -> Result<(), super::super::super::_TrappableError0> {
                        Host::set_many(*self, bucket, key_values).await
                    }
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
                    async fn delete_many(
                        &mut self,
                        bucket: wasmtime::component::Resource<Bucket>,
                        keys: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> Result<(), super::super::super::_TrappableError0> {
                        Host::delete_many(*self, bucket, keys).await
                    }
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
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type Bucket = super::super::super::super::wasi::keyvalue::store::Bucket;
                    pub struct Guest {
                        on_set: wasmtime::component::Func,
                        on_delete: wasmtime::component::Func,
                    }
                    pub struct GuestIndices {
                        on_set: wasmtime::component::ComponentExportIndex,
                        on_delete: wasmtime::component::ComponentExportIndex,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for GuestIndices {
                        #[inline]
                        fn clone(&self) -> GuestIndices {
                            GuestIndices {
                                on_set: ::core::clone::Clone::clone(&self.on_set),
                                on_delete: ::core::clone::Clone::clone(&self.on_delete),
                            }
                        }
                    }
                    impl GuestIndices {
                        /// Constructor for [`GuestIndices`] which takes a
                        /// [`Component`](wasmtime::component::Component) as input and can be executed
                        /// before instantiation.
                        ///
                        /// This constructor can be used to front-load string lookups to find exports
                        /// within a component.
                        pub fn new(
                            component: &wasmtime::component::Component,
                        ) -> wasmtime::Result<GuestIndices> {
                            let (_, instance) = component
                                .export_index(None, "wasi:keyvalue/watcher@0.2.0-draft")
                                .ok_or_else(|| ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "no exported instance named `wasi:keyvalue/watcher@0.2.0-draft`",
                                        ),
                                    );
                                    error
                                }))?;
                            Self::_new(|name| {
                                component.export_index(Some(&instance), name).map(|p| p.1)
                            })
                        }
                        /// This constructor is similar to [`GuestIndices::new`] except that it
                        /// performs string lookups after instantiation time.
                        pub fn new_instance(
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<GuestIndices> {
                            let instance_export = instance
                                .get_export(
                                    &mut store,
                                    None,
                                    "wasi:keyvalue/watcher@0.2.0-draft",
                                )
                                .ok_or_else(|| ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "no exported instance named `wasi:keyvalue/watcher@0.2.0-draft`",
                                        ),
                                    );
                                    error
                                }))?;
                            Self::_new(|name| {
                                instance
                                    .get_export(&mut store, Some(&instance_export), name)
                            })
                        }
                        fn _new(
                            mut lookup: impl FnMut(
                                &str,
                            ) -> Option<wasmtime::component::ComponentExportIndex>,
                        ) -> wasmtime::Result<GuestIndices> {
                            let mut lookup = move |name| {
                                lookup(name)
                                    .ok_or_else(|| {
                                        ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!(
                                                    "instance export `wasi:keyvalue/watcher@0.2.0-draft` does not have export `{0}`",
                                                    name,
                                                ),
                                            );
                                            error
                                        })
                                    })
                            };
                            let _ = &mut lookup;
                            let on_set = lookup("on-set")?;
                            let on_delete = lookup("on-delete")?;
                            Ok(GuestIndices { on_set, on_delete })
                        }
                        pub fn load(
                            &self,
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Guest> {
                            let mut store = store.as_context_mut();
                            let _ = &mut store;
                            let _instance = instance;
                            let on_set = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::Resource<Bucket>, &str, &[u8]),
                                    (),
                                >(&mut store, &self.on_set)?
                                .func();
                            let on_delete = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::Resource<Bucket>, &str),
                                    (),
                                >(&mut store, &self.on_delete)?
                                .func();
                            Ok(Guest { on_set, on_delete })
                        }
                    }
                    impl Guest {
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
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "services::keyvalue::generated::exports::wasi::keyvalue::watcher",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::exports::wasi::keyvalue::watcher",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"wasi:keyvalue/watcher@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"on-set" as &dyn Value,
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
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<Bucket>, &str, &[u8]),
                                    (),
                                >::new_unchecked(self.on_set)
                            };
                            let () = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .instrument(span.clone())
                                .await?;
                            callee
                                .post_return_async(store.as_context_mut())
                                .instrument(span)
                                .await?;
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
                            use tracing::Instrument;
                            let span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "wit-bindgen export",
                                            "services::keyvalue::generated::exports::wasi::keyvalue::watcher",
                                            tracing::Level::TRACE,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/services/src/keyvalue.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(13u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "services::keyvalue::generated::exports::wasi::keyvalue::watcher",
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
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"wasi:keyvalue/watcher@0.2.0-draft" as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &"on-delete" as &dyn Value,
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
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<Bucket>, &str),
                                    (),
                                >::new_unchecked(self.on_delete)
                            };
                            let () = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .instrument(span.clone())
                                .await?;
                            callee
                                .post_return_async(store.as_context_mut())
                                .instrument(span)
                                .await?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }
    const _: &str = "package wasi:vault@0.1.0-draft;\n\nworld imports {\n\timport keystore;\n}\n\nworld vault {\n\tinclude imports;\n}\n";
    const _: &str = "\ninterface types {\n    /// A serialized request to a RPC server.\n    type request = list<u8>;\n\n    /// A serialized response from a RPC server.\n    type response = list<u8>;\n}\n\n/// A psuedo-RPC Client interface that can be used to make requests to a wRPC server.\n/// \ninterface client {\n    use types.{request, response};\n\n    /// Error type that can be returned by implementing runtime. The runtime should provide a \n    /// trace of the error.\n    resource error {\n        trace: func() -> string;    \n    }\n\n    /// Call the specified endpoint.\n    /// \n    /// `endpoint` identifies the server identifier + method to call. \n    /// \n    /// For example, \n    /// \n    /// ```rust\n    /// let request = serde_json::to_vec(&ClaimsRequest {\n    ///     claims: ...,\n    /// })?;\n    /// \n    /// let resp = wrpc::client::call(\"holder.v1.HolderService/GetClaims\", request)?;\n    /// ...\n    /// ```\n    ///\n    /// `request` is a serialized server-specific request object.\n    call: func(endpoint: string, request: request) -> result<response, error>;\n}\n\n/// A psuedo-RPC Server interface that can be used to serve wRPC clients (above).\ninterface server {\n    use types.{request, response};\n\n    /// Errors returned by the server.\n    variant error {\n        /// The endpoint specified is invalid.\n        unknown-endpoint,\n\n        /// The request was invalid.\n        invalid-request,\n\n        /// Some implementation-specific error occurred.\n        other(string)\n    }\n\n    /// Configuration includes the server\'s unique identifier.\n    record server-configuration {\n        identifier: string,\n    }\n\n    /// Configure is called by the runtime to get the server\'s runtime configuration.\n    /// At present, this consists of the server\'s `identifier` as used by the Client \n    /// when calling a server.\n    configure: func() -> result<server-configuration, error>;\n\n    /// Handle a request from a client.\n    handle: func(endpoint: string, request: request) -> result<response, error>;\n}\n";
    const _: &str = "/// https://component-model.bytecodealliance.org/design/wit.html\n\n/// Keystore provides an interface for secure storage and use of cryptographic \n/// keys.\ninterface keystore {\n    variant error {\n        no-such-key-set,\n        no-such-key-pair,\n        access-denied,\n        other(string)\n    }\n\n    /// [IANA_JOSE] Keystore algorithms.\n    /// [IANA_JOSE]: https://www.iana.org/assignments/jose/jose.xhtml\n    variant algorithm {\n        /// ECDSA using `secp256k1` curve and SHA-256. \n        es256k,\n\n        /// EdDSA vault algorithms using `ed22519` curve and SHA-256.\n        eddsa,\n    }\n\n    /// JSON Web Key format public key.\n    /// See <https://www.rfc-editor.org/rfc/rfc7517.html>.\n    record jwk {\n        kid: option<string>,\n        kty: string,\n        crv: string,\n        x: string,\n        y: option<string>,\n    }\n\n    /// List algorithms supported by the keystore.\n    supported-algorithms: func() -> list<algorithm>;\n\n    /// Open the key set identified by `identifier`.\n    open: func(identifier: string) -> result<key-set, error>;\n\n    resource key-set {\n        /// Generate a new key pair for the set. Will create a new version if the\n        /// key pair identified by `identifier` already exists.\n        generate: func(identifier: string, alg: algorithm) -> result<key-pair, error>;\n\n        /// Get the current key pair for `identifier`.\n        get: func(identifier: string) -> result<key-pair, error>;\n\n        /// Delete the key pair identified by `identifier`.\n        delete: func(identifier: string) -> result<_, error>;\n    }\n\n    resource key-pair {\n        /// Sign data with the private key.\n        sign: func(data: list<u8>) -> result<list<u8>, error>;\n\n        /// Returns the public key.\n        public-key: func() -> result<jwk, error>;\n\n        /// Returns all versions of the public key.\n        versions: func() -> result<list<jwk>, error>;\n    }\n}\n";
    const _: &str = "interface readwrite {\n    use types.{database, error, statement};\n    \n    /// insert.\n    insert: func(db: borrow<database>, s: borrow<statement>, d: list<u8>) -> result<_, error>;\n\n    /// query.\n    find: func(db: borrow<database>, s: borrow<statement>) -> result<list<list<u8>>, error>;\n\n    /// update.\n    update: func(db: borrow<database>, s: borrow<statement>, d: list<u8>) -> result<_, error>;\n\n    /// delete.\n    delete: func(db: borrow<database>, s: borrow<statement>) -> result<_, error>;\n}";
    const _: &str = "package wasi:rpc@0.1.0-draft;\n\nworld imports {\n    import types;\n\timport client;\n}\n\nworld rpc {\n\tinclude imports;\n\texport server;\n}";
    const _: &str = "/// A keyvalue interface that provides watch operations.\n/// \n/// This interface is used to provide event-driven mechanisms to handle\n/// keyvalue changes.\ninterface watcher {\n\t/// A keyvalue interface that provides handle-watch operations.\n\tuse store.{bucket};\n\n\t/// Handle the `set` event for the given bucket and key. It includes a reference to the `bucket`\n\t/// that can be used to interact with the store.\n\ton-set: func(bucket: bucket, key: string, value: list<u8>);\n\n\t/// Handle the `delete` event for the given bucket and key. It includes a reference to the\n\t/// `bucket` that can be used to interact with the store.\n\ton-delete: func(bucket: bucket, key: string);\n}";
    const _: &str = "package wasi:jsondb@0.1.0-draft;\n\nworld imports {\n\timport readwrite;\n}";
    const _: &str = "interface messaging-guest {\n    use messaging-types.{message, guest-configuration, error};\n\n    /// Returns the list of channels (and extension metadata within guest-configuration) that \n    /// this component should subscribe to and be handled by the subsequent handler within guest-configuration\n    configure: func() -> result<guest-configuration, error>;\n\n    /// Whenever this guest receives a message in one of the subscribed channels, the message is sent to this handler\n    handler: func(ms: list<message>) -> result<_, error>;\n}\n";
    const _: &str = "interface messaging-types {\n    /// A connection to a message-exchange service (e.g., buffer, broker, etc.).\n    resource client {\n        connect: static func(name: string) -> result<client, error>;\n    }\n    \n    /// TODO(danbugs): This should be eventually extracted as an underlying type for other wasi-cloud-core interfaces.\n    resource error {\n        trace: static func() -> string;    \n    }\n  \n    /// There are two types of channels:\n    /// - publish-subscribe channel, which is a broadcast channel, and\n    /// - point-to-point channel, which is a unicast channel.\n    ///\n    /// The interface doesn\'t highlight this difference in the type itself as that\'s uniquely a consumer issue.\n    type channel = string;\n  \n    /// Configuration includes a required list of channels the guest is subscribing to, and an optional list of extensions key-value pairs \n    /// (e.g., partitions/offsets to read from in Kafka/EventHubs, QoS etc.).\n    record guest-configuration {\n        channels: list<channel>,\n        extensions: option<list<tuple<string, string>>>\n    }\n  \n    /// Format specification for messages \n    ///  - more info: https://github.com/clemensv/spec/blob/registry-extensions/registry/spec.md#message-formats\n    ///  - message metadata can further decorate w/ things like format version, and so on.\n    enum format-spec {\n        cloudevents,\n        http,\n        amqp,\n        mqtt,\n        kafka,\n        raw\n    }\n  \n    /// A message with a binary payload, a format specification, and decorative metadata.\n    record message {\n        data: list<u8>,\n        format: format-spec,\n        metadata: option<list<tuple<string, string>>>\n    }\n}";
    const _: &str = "/// Operations on a container of entries that make up a blob.\ninterface container {\n    use types.{container-id, error, author, token, entry-metadata, permission};\n\n    /// Create a new authoer on the underlying node and receive an ID to use for document writing.\n    create-author: func() -> result<author, error>;\n\n    /// Create a new container where the container handle is owned by the owner.\n    create-container: func() -> result<container, error>;\n\n    /// Retrieve an existing container using a token (which is imbued with read/write permissions).\n    get-container: func(token: token) -> result<container, error>;\n\n    /// Delete a container and all objects within it\n    delete-container: func(container: container) -> result<_, error>;\n\n    /// A container is a collection of entries that make up a blob.\n    /// \n    /// 1. Iroh calls this a document.\n    resource container {\n        /// Return the container identifier.\n        id: func() -> result<container-id, error>;\n\n        /// Write an entry to the container.\n        write-entry: func(\n            /// Key of the entry to write to.\n            key: string,\n            /// ID of the author making the entry.\n            author: author,\n            /// Data to be used as the entry.\n            data: list<u8>) -> result<_, error>;\n\n        /// List entry keys in the container.\n        list-entries: func() -> result<list<string>, error>;\n\n        /// Get entry metadata for an exact key match. Returns an error if the entry does not exist.\n        get-entry-metadata: func(key: string) -> result<entry-metadata, error>;\n\n        /// Read an entry from the container from byte positions `start` up to `len` bytes.\n        read-entry: func(key: string, start: u64, len: u64) -> result<list<u8>, error>;\n\n        /// Remove an entry from the container.\n        delete-entry: func(key: string) -> result<_, error>;\n\n        /// Remove all entries from the container.\n        clear-entries: func() -> result<_, error>;\n\n        /// Get a token that gives access to the container.\n        get-token: func(permission: permission) -> result<token, error>;\n    }\n}\n";
    const _: &str = "/// A keyvalue interface that provides batch operations.\n/// \n/// A batch operation is an operation that operates on multiple keys at once.\n/// \n/// Batch operations are useful for reducing network round-trip time. For example, if you want to\n/// get the values associated with 100 keys, you can either do 100 get operations or you can do 1\n/// batch get operation. The batch operation is faster because it only needs to make 1 network call\n/// instead of 100.\n/// \n/// A batch operation does not guarantee atomicity, meaning that if the batch operation fails, some\n/// of the keys may have been modified and some may not. \n/// \n/// This interface does has the same consistency guarantees as the `store` interface, meaning that\n/// you should be able to \"read your writes.\"\n/// \n/// Please note that this interface is bare functions that take a reference to a bucket. This is to\n/// get around the current lack of a way to \"extend\" a resource with additional methods inside of\n/// wit. Future version of the interface will instead extend these methods on the base `bucket`\n/// resource.\ninterface batch {\n    use store.{bucket, error};\n\n    /// Get the key-value pairs associated with the keys in the store. It returns a list of\n    /// key-value pairs.\n    ///\n    /// If any of the keys do not exist in the store, it returns a `none` value for that pair in the\n    /// list.\n    /// \n    /// MAY show an out-of-date value if there are concurrent writes to the store.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`.\n    get-many: func(bucket: borrow<bucket>, keys: list<string>) -> result<list<option<tuple<string, list<u8>>>>, error>;\n\n    /// Set the values associated with the keys in the store. If the key already exists in the\n    /// store, it overwrites the value. \n    /// \n    /// Note that the key-value pairs are not guaranteed to be set in the order they are provided. \n    ///\n    /// If any of the keys do not exist in the store, it creates a new key-value pair.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not\n    /// rollback the key-value pairs that were already set. Thus, this batch operation does not\n    /// guarantee atomicity, implying that some key-value pairs could be set while others might\n    /// fail. \n    /// \n    /// Other concurrent operations may also be able to see the partial results.\n    set-many: func(bucket: borrow<bucket>, key-values: list<tuple<string, list<u8>>>) -> result<_, error>;\n\n    /// Delete the key-value pairs associated with the keys in the store.\n    /// \n    /// Note that the key-value pairs are not guaranteed to be deleted in the order they are\n    /// provided.\n    /// \n    /// If any of the keys do not exist in the store, it skips the key.\n    /// \n    /// If any other error occurs, it returns an `Err(error)`. When an error occurs, it does not\n    /// rollback the key-value pairs that were already deleted. Thus, this batch operation does not\n    /// guarantee atomicity, implying that some key-value pairs could be deleted while others might\n    /// fail.\n    /// \n    /// Other concurrent operations may also be able to see the partial results.\n    delete-many: func(bucket: borrow<bucket>, keys: list<string>) -> result<_, error>;\n}\n";
    const _: &str = "package wasi:messaging@0.2.0-draft;\n\nworld imports {\n\timport producer;\n\timport consumer;\n}\n\nworld messaging {\n\tinclude imports;\n\texport messaging-guest;\n}";
    const _: &str = "interface types {\n    /// A database connection.\n    resource database {\n        connect: static func(name: string) -> result<database, error>;\n    }\n\n    /// A JSON database statement to be used in database operations.\n    /// \n    /// - `collection`: The name of the collection of JSON documents to be \n    ///   used in the operation.\n    /// \n    /// - `jmes-path`: The JMESPath filter expresession to be applied by the \n    ///   operation. For example, [?firstName==\'John\'] will return \n    ///   all documents where the `firstName` field is equal to `John`.\n    resource statement {\n        prepare: static func(collection: string, jmes-path: option<string>) -> result<statement, error>;\n    }\n\n    /// An error resource type.\n    resource error {\n\t\ttrace: func() -> string;\n  \t}\n}";
    const _: &str = "package wasi:keyvalue@0.2.0-draft;\n\n/// The `wasi:keyvalue/imports` world provides common APIs for interacting with key-value stores.\n/// Components targeting this world will be able to do:\n/// \n/// 1. CRUD (create, read, update, delete) operations on key-value stores.\n/// 2. Atomic `increment` and CAS (compare-and-swap) operations.\n/// 3. Batch operations that can reduce the number of round trips to the network.\nworld imports {\n\t/// The `store` service allows the component to perform eventually consistent operations on\n\t/// the key-value store.\n\timport store;\n\n\t/// The `atomic` service allows the component to perform atomic / `increment` and CAS\n\t/// (compare-and-swap) operations.\n\timport atomics;\n\n\t/// The `batch` service allows the component to perform eventually consistent batch\n\t/// operations that can reduce the number of round trips to the network.\n\timport batch;\n}\n\nworld watch-service {\n\tinclude imports;\n\texport watcher;\n}";
    const _: &str = "interface producer {\n    use messaging-types.{client, channel, message, error};\n    \n    send: func(c: client, ch: channel, m: list<message>) -> result<_, error>;\n}\n";
    const _: &str = "interface consumer {\n    // {client, message, channel, error, guest-configuration}\n    use messaging-types.{client, message, channel, error, guest-configuration};\n\n    /// Blocking receive for t-milliseconds with ephemeral subscription \u{2013}\u{a0}if no message is received, returns None\n    subscribe-try-receive: func(c: client, ch: channel, t-milliseconds: u32) -> result<option<list<message>>, error>;\n\n    /// Blocking receive until message with ephemeral subscription\n    subscribe-receive: func(c: client, ch: channel) -> result<list<message>, error>;\n\n    /// \'Fit-all\' type function for updating a guest\'s configuration \u{2013} this could be useful for:\n    ///     - unsubscribing from a channel,\n    ///     - checkpointing,\n    ///     - etc..\n    update-guest-configuration: func(gc: guest-configuration) -> result<_, error>;\n\n    /// A message can exist under several statuses:\n    /// (1) available: the message is ready to be read,\n    /// (2) acquired: the message has been sent to a consumer (but still exists in the queue),\n    /// (3) accepted (result of complete-message): the message has been received and ACK-ed by a consumer and can be safely removed from the queue,\n    /// (4) rejected (result of abandon-message): the message has been received and NACK-ed by a consumer, at which point it can be:\n    ///         - deleted,\n    ///         - sent to a dead-letter queue, or\n    ///         - kept in the queue for further processing.\n    complete-message: func(m: message) -> result<_, error>;\n    abandon-message: func(m: message) -> result<_, error>;\n}\n";
    const _: &str = "package wasi:p2p@0.1.0-draft;\n\nworld imports {\n    import container;\n}\n\nworld p2p {\n    include imports;\n}";
    const _: &str = "package wasmgrid:service;\n\nworld jsondb {\n\tinclude wasi:jsondb/imports@0.1.0-draft;\n}\n\nworld keyvalue {\n\tinclude wasi:keyvalue/imports@0.2.0-draft;\n\texport wasi:keyvalue/watcher@0.2.0-draft;\n}\n\nworld messaging {\n\tinclude wasi:messaging/imports@0.2.0-draft;\n\texport wasi:messaging/messaging-guest@0.2.0-draft;\n}\n\nworld vault {\n\tinclude wasi:vault/imports@0.1.0-draft;\n}\n\n// world p2p {\n// \tinclude wasi:p2p/imports@0.1.0-draft;\n// }\n\nworld rpc {\n\tinclude wasi:rpc/imports@0.1.0-draft;\n\texport wasi:rpc/server@0.1.0-draft;\n}";
    const _: &str = "/// A keyvalue interface that provides eventually consistent key-value operations.\n/// \n/// Each of these operations acts on a single key-value pair.\n/// \n/// The value in the key-value pair is defined as a `u8` byte array and the intention is that it is\n/// the common denominator for all data types defined by different key-value stores to handle data,\n/// ensuring compatibility between different key-value stores. Note: the clients will be expecting\n/// serialization/deserialization overhead to be handled by the key-value store. The value could be\n/// a serialized object from JSON, HTML or vendor-specific data types like AWS S3 objects.\n/// \n/// Data consistency in a key value store refers to the guarantee that once a write operation\n/// completes, all subsequent read operations will return the value that was written.\n/// \n/// Any implementation of this interface must have enough consistency to guarantee \"reading your\n/// writes.\" In particular, this means that the client should never get a value that is older than\n/// the one it wrote, but it MAY get a newer value if one was written around the same time. These\n/// guarantees only apply to the same client (which will likely be provided by the host or an\n/// external service of some kind). In this context a \"client\" is referring to the caller or\n/// guest that is consuming this interface. Once a write request is committed by a specific client,\n/// all subsequent read requests by the same client will reflect that write or any subsequent\n/// writes. Another client running in a different context may or may not immediately see the result\n/// due to the replication lag. As an example of all of this, if a value at a given key is A, and\n/// the client writes B, then immediately reads, it should get B. If something else writes C in\n/// quick succession, then the client may get C. However, a client running in a separate context may\n/// still see A or B\ninterface store {\n    /// The set of errors which may be raised by functions in this package\n    variant error {\n        /// The host does not recognize the store identifier requested.\n        no-such-store,\n\n        /// The requesting component does not have access to the specified store\n        /// (which may or may not exist).\n        access-denied,\n\n        /// Some implementation-specific error has occurred (e.g. I/O)\n        other(string)\n    }\n\n    /// A response to a `list-keys` operation.\n    record key-response {\n        /// The list of keys returned by the query.\n        keys: list<string>,\n        /// The continuation token to use to fetch the next page of keys. If this is `null`, then\n        /// there are no more keys to fetch.\n        cursor: option<u64>\n    }\n\n    /// Get the bucket with the specified identifier.\n    ///\n    /// `identifier` must refer to a bucket provided by the host.\n    ///\n    /// `error::no-such-store` will be raised if the `identifier` is not recognized.\n    open: func(identifier: string) -> result<bucket, error>;\n\n    /// A bucket is a collection of key-value pairs. Each key-value pair is stored as a entry in the\n    /// bucket, and the bucket itself acts as a collection of all these entries.\n    ///\n    /// It is worth noting that the exact terminology for bucket in key-value stores can very\n    /// depending on the specific implementation. For example:\n    ///\n    /// 1. Amazon DynamoDB calls a collection of key-value pairs a table\n    /// 2. Redis has hashes, sets, and sorted sets as different types of collections\n    /// 3. Cassandra calls a collection of key-value pairs a column family\n    /// 4. MongoDB calls a collection of key-value pairs a collection\n    /// 5. Riak calls a collection of key-value pairs a bucket\n    /// 6. Memcached calls a collection of key-value pairs a slab\n    /// 7. Azure Cosmos DB calls a collection of key-value pairs a container\n    ///\n    /// In this interface, we use the term `bucket` to refer to a collection of key-value pairs\n    resource bucket {\n        /// Get the value associated with the specified `key`\n        ///\n        /// The value is returned as an option. If the key-value pair exists in the\n        /// store, it returns `Ok(value)`. If the key does not exist in the\n        /// store, it returns `Ok(none)`. \n        ///\n        /// If any other error occurs, it returns an `Err(error)`.\n        get: func(key: string) -> result<option<list<u8>>, error>;\n\n        /// Set the value associated with the key in the store. If the key already\n        /// exists in the store, it overwrites the value.\n        ///\n        /// If the key does not exist in the store, it creates a new key-value pair.\n        /// \n        /// If any other error occurs, it returns an `Err(error)`.\n        set: func(key: string, value: list<u8>) -> result<_, error>;\n\n        /// Delete the key-value pair associated with the key in the store.\n        /// \n        /// If the key does not exist in the store, it does nothing.\n        ///\n        /// If any other error occurs, it returns an `Err(error)`.\n        delete: func(key: string) -> result<_, error>;\n\n        /// Check if the key exists in the store.\n        /// \n        /// If the key exists in the store, it returns `Ok(true)`. If the key does\n        /// not exist in the store, it returns `Ok(false)`.\n        /// \n        /// If any other error occurs, it returns an `Err(error)`.\n        exists: func(key: string) -> result<bool, error>;\n\n        /// Get all the keys in the store with an optional cursor (for use in pagination). It\n        /// returns a list of keys. Please note that for most KeyValue implementations, this is a\n        /// can be a very expensive operation and so it should be used judiciously. Implementations\n        /// can return any number of keys in a single response, but they should never attempt to\n        /// send more data than is reasonable (i.e. on a small edge device, this may only be a few\n        /// KB, while on a large machine this could be several MB). Any response should also return\n        /// a cursor that can be used to fetch the next page of keys. See the `key-response` record\n        /// for more information.\n        /// \n        /// Note that the keys are not guaranteed to be returned in any particular order.\n        /// \n        /// If the store is empty, it returns an empty list.\n        /// \n        /// MAY show an out-of-date list of keys if there are concurrent writes to the store.\n        /// \n        /// If any error occurs, it returns an `Err(error)`.\n        list-keys: func(cursor: option<u64>) -> result<key-response, error>;\n    }\n}\n";
    const _: &str = "/// A keyvalue interface that provides atomic operations.\n/// \n/// Atomic operations are single, indivisible operations. When a fault causes an atomic operation to\n/// fail, it will appear to the invoker of the atomic operation that the action either completed\n/// successfully or did nothing at all.\n/// \n/// Please note that this interface is bare functions that take a reference to a bucket. This is to\n/// get around the current lack of a way to \"extend\" a resource with additional methods inside of\n/// wit. Future version of the interface will instead extend these methods on the base `bucket`\n/// resource.\ninterface atomics {\n  \tuse store.{bucket, error};\n\n  \t/// Atomically increment the value associated with the key in the store by the given delta. It\n\t/// returns the new value.\n\t///\n\t/// If the key does not exist in the store, it creates a new key-value pair with the value set\n\t/// to the given delta. \n\t///\n\t/// If any other error occurs, it returns an `Err(error)`.\n\tincrement: func(bucket: borrow<bucket>, key: string, delta: u64) -> result<u64, error>;\n}";
    const _: &str = "/// Types used by a peer-to-peer node.\ninterface types {\n    /// The set of errors that may be raised by functions in this package.\n    variant error {\n        /// The service providing the service cannot be used.\n        service-unavailable,\n        /// The service providing the service returned an error.\n        service-error(string),\n        /// An error occurred while trying to read an entry stream.\n        read-error(string),\n        /// An error occurred while trying to write an entry stream.\n        write-error(string),\n        /// No container with the specified identifier exists.\n        no-such-container,\n        /// No entry with the specified key exists in the container.\n        no-such-entry,\n        /// An unspecified error occurred.\n        other(string),\n    }\n\n    /// Container identifier.\n    type container-id = string;\n\n    /// Public key of a container author.\n    /// \n    /// 1. Iroh calls this a short author ID that can be resolved to an author public key.\n    type author = string;\n\n    /// Token that gives access to a container.\n    /// \n    /// 1. Iroh calls this a ticket.\n    type token = string;\n\n    /// Permissions that holder of a container token can have.\n    variant permission {\n        /// Holder can read the container.\n        read,\n\n        /// Holder can write to the container. Write implies read.\n        write,\n    }\n\n    /// Information about an entry in a container.\n    record entry-metadata {\n        /// Name of the entry.\n        name: string,\n        /// Name of the container the entry is in.\n        container: string,\n        /// Size of the entry in bytes.\n        size: u64,\n        /// Time the entry was created (seconds since Unix epoch).\n        created-at: u64,\n        /// Author of the entry.\n        author: author,\n    }\n}\n";
}
