//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn instrument(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    let name = item.sig.ident.clone();
    let inputs = item.sig.inputs.clone();
    let output = item.sig.output.clone();
    let block = item.block.clone();

    // Create a new function with the same name and block
    let new_fn = quote::quote! {
        #[allow(non_snake_case)]
        fn #name(#inputs) #output {
            sdk_otel::instrument(|| {
                #block
            });
        }
    };

    TokenStream::from(new_fn)
}
