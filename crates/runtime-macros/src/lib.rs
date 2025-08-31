//! # Wasmgrid Runtime Macros

mod runtime;

use proc_macro::TokenStream;
use syn::{Error, parse_macro_input};

use self::runtime::Json;

#[proc_macro]
pub fn runtime(input: TokenStream) -> TokenStream {
    runtime::generate(&parse_macro_input!(input as Json))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
