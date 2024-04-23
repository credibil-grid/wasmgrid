// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod messaging {
        #[allow(dead_code, clippy::all)]
        pub mod messaging_types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// A connection to a message-exchange service (e.g., buffer, broker, etc.).

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Client {
                handle: _rt::Resource<Client>,
            }

            impl Client {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Client {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:messaging/messaging-types@0.2.0-draft")]
                        extern "C" {
                            #[link_name = "[resource-drop]client"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            /// TODO(danbugs): This should be eventually extracted as an underlying type for other wasi-cloud-core interfaces.

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error {
                handle: _rt::Resource<Error>,
            }

            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:messaging/messaging-types@0.2.0-draft")]
                        extern "C" {
                            #[link_name = "[resource-drop]error"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            /// There are two types of channels:
            /// - publish-subscribe channel, which is a broadcast channel, and
            /// - point-to-point channel, which is a unicast channel.
            ///
            /// The interface doesn't highlight this difference in the type itself as that's uniquely a consumer issue.
            pub type Channel = _rt::String;
            /// Configuration includes a required list of channels the guest is subscribing to, and an optional list of extensions key-value pairs
            /// (e.g., partitions/offsets to read from in Kafka/EventHubs, QoS etc.).
            #[derive(Clone)]
            pub struct GuestConfiguration {
                pub channels: _rt::Vec<Channel>,
                pub extensions: Option<_rt::Vec<(_rt::String, _rt::String)>>,
            }
            impl ::core::fmt::Debug for GuestConfiguration {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("GuestConfiguration")
                        .field("channels", &self.channels)
                        .field("extensions", &self.extensions)
                        .finish()
                }
            }
            /// Format specification for messages
            /// - more info: https://github.com/clemensv/spec/blob/registry-extensions/registry/spec.md#message-formats
            /// - message metadata can further decorate w/ things like format version, and so on.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum FormatSpec {
                Cloudevents,
                Http,
                Amqp,
                Mqtt,
                Kafka,
                Raw,
            }
            impl ::core::fmt::Debug for FormatSpec {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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

            impl FormatSpec {
                pub(crate) unsafe fn _lift(val: u8) -> FormatSpec {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => FormatSpec::Cloudevents,
                        1 => FormatSpec::Http,
                        2 => FormatSpec::Amqp,
                        3 => FormatSpec::Mqtt,
                        4 => FormatSpec::Kafka,
                        5 => FormatSpec::Raw,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            /// A message with a binary payload, a format specification, and decorative metadata.
            #[derive(Clone)]
            pub struct Message {
                pub data: _rt::Vec<u8>,
                pub format: FormatSpec,
                pub metadata: Option<_rt::Vec<(_rt::String, _rt::String)>>,
            }
            impl ::core::fmt::Debug for Message {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Message")
                        .field("data", &self.data)
                        .field("format", &self.format)
                        .field("metadata", &self.metadata)
                        .finish()
                }
            }
            impl Client {
                #[allow(unused_unsafe, clippy::all)]
                pub fn connect(name: &str) -> Result<Client, Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:messaging/messaging-types@0.2.0-draft")]
                        extern "C" {
                            #[link_name = "[static]client.connect"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<i32>();

                                    Client::from_handle(l3 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = *ptr1.add(4).cast::<i32>();

                                    Error::from_handle(l4 as u32)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                pub fn trace() -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:messaging/messaging-types@0.2.0-draft")]
                        extern "C" {
                            #[link_name = "[static]error.trace"]
                            fn wit_import(_: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8) {
                            unreachable!()
                        }
                        wit_import(ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }

        #[allow(dead_code, clippy::all)]
        pub mod producer {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Client = super::super::super::wasi::messaging::messaging_types::Client;
            pub type Channel = super::super::super::wasi::messaging::messaging_types::Channel;
            pub type Message = super::super::super::wasi::messaging::messaging_types::Message;
            pub type Error = super::super::super::wasi::messaging::messaging_types::Error;
            #[allow(unused_unsafe, clippy::all)]
            pub fn send(c: Client, ch: &Channel, m: &[Message]) -> Result<(), Error> {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = ch;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let vec7 = m;
                    let len7 = vec7.len();
                    let layout7 = _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 24, 4);
                    let result7 = if layout7.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout7);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7.add(i * 24);
                        {
                            let super::super::super::wasi::messaging::messaging_types::Message {
                                data: data1,
                                format: format1,
                                metadata: metadata1,
                            } = e;
                            let vec2 = data1;
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            *base.add(4).cast::<usize>() = len2;
                            *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                            *base.add(8).cast::<u8>() = (format1.clone() as i32) as u8;
                            match metadata1 {
                                Some(e) => {
                                    *base.add(12).cast::<u8>() = (1i32) as u8;
                                    let vec6 = e;
                                    let len6 = vec6.len();
                                    let layout6 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec6.len() * 16,
                                        4,
                                    );
                                    let result6 = if layout6.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout6).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout6);
                                        }
                                        ptr
                                    } else {
                                        {
                                            ::core::ptr::null_mut()
                                        }
                                    };
                                    for (i, e) in vec6.into_iter().enumerate() {
                                        let base = result6.add(i * 16);
                                        {
                                            let (t3_0, t3_1) = e;
                                            let vec4 = t3_0;
                                            let ptr4 = vec4.as_ptr().cast::<u8>();
                                            let len4 = vec4.len();
                                            *base.add(4).cast::<usize>() = len4;
                                            *base.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                                            let vec5 = t3_1;
                                            let ptr5 = vec5.as_ptr().cast::<u8>();
                                            let len5 = vec5.len();
                                            *base.add(12).cast::<usize>() = len5;
                                            *base.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                        }
                                    }
                                    *base.add(20).cast::<usize>() = len6;
                                    *base.add(16).cast::<*mut u8>() = result6;
                                    cleanup_list.extend_from_slice(&[(result6, layout6)]);
                                }
                                None => {
                                    *base.add(12).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                    }
                    let ptr8 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:messaging/producer@0.2.0-draft")]
                    extern "C" {
                        #[link_name = "send"]
                        fn wit_import(
                            _: i32, _: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8,
                        );
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        (&c).take_handle() as i32,
                        ptr0.cast_mut(),
                        len0,
                        result7,
                        len7,
                        ptr8,
                    );
                    let l9 = i32::from(*ptr8.add(0).cast::<u8>());
                    if layout7.size() != 0 {
                        _rt::alloc::dealloc(result7.cast(), layout7);
                    }
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                    match l9 {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l10 = *ptr8.add(4).cast::<i32>();

                                super::super::super::wasi::messaging::messaging_types::Error::from_handle(l10 as u32)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
mod _rt {

    use core::sync::atomic::AtomicU32;
    use core::sync::atomic::Ordering::Relaxed;
    use core::{fmt, marker};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:messaging:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 691] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb3\x04\x01A\x02\x01\
A\x08\x01B\x16\x04\0\x06client\x03\x01\x04\0\x05error\x03\x01\x01s\x04\0\x07chan\
nel\x03\0\x02\x01p\x03\x01o\x02ss\x01p\x05\x01k\x06\x01r\x02\x08channels\x04\x0a\
extensions\x07\x04\0\x13guest-configuration\x03\0\x08\x01m\x06\x0bcloudevents\x04\
http\x04amqp\x04mqtt\x05kafka\x03raw\x04\0\x0bformat-spec\x03\0\x0a\x01p}\x01r\x03\
\x04data\x0c\x06format\x0b\x08metadata\x07\x04\0\x07message\x03\0\x0d\x01i\0\x01\
i\x01\x01j\x01\x0f\x01\x10\x01@\x01\x04names\0\x11\x04\0\x16[static]client.conne\
ct\x01\x12\x01@\0\0s\x04\0\x13[static]error.trace\x01\x13\x03\x01*wasi:messaging\
/messaging-types@0.2.0-draft\x05\0\x02\x03\0\0\x06client\x02\x03\0\0\x07channel\x02\
\x03\0\0\x07message\x02\x03\0\0\x05error\x01B\x0e\x02\x03\x02\x01\x01\x04\0\x06c\
lient\x03\0\0\x02\x03\x02\x01\x02\x04\0\x07channel\x03\0\x02\x02\x03\x02\x01\x03\
\x04\0\x07message\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x05error\x03\0\x06\x01i\x01\
\x01p\x05\x01i\x07\x01j\0\x01\x0a\x01@\x03\x01c\x08\x02ch\x03\x01m\x09\0\x0b\x04\
\0\x04send\x01\x0c\x03\x01#wasi:messaging/producer@0.2.0-draft\x05\x05\x04\x01\x1c\
component:http-msg/messaging\x04\0\x0b\x0f\x01\0\x09messaging\x03\0\0\0G\x09prod\
ucers\x01\x0cprocessed-by\x02\x0dwit-component\x070.202.0\x10wit-bindgen-rust\x06\
0.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
