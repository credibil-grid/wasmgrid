//! # OpenTelemetry Attribute Macros

use proc_macro::TokenStream;
use quote::quote;
use syn::meta::{self, ParseNestedMeta};
use syn::parse::Result;
use syn::{Expr, ItemFn, LitStr, parse_macro_input};

/// Instruments a function using the `[sdk_otel::instrument]` function.
///
/// This macro can be used to automatically create spans for functions, making
/// it easier to add observability to your code.
///
/// ```
/// use opentelemetry::{KeyValue, global};
///
/// #[sdk_otel_attr::instrument(name = "my_span")]
/// fn my_function() {
///     let meter = global::meter("my_meter");
///     let counter = meter.u64_counter("my_counter").build();
///     counter.add(1, &[KeyValue::new("key1", "value 1")]);
///
///     tracing::info_span!("info span").in_scope(|| {
///         tracing::info!("info event");
///     });
///
///     // function body
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument(args: TokenStream, item: TokenStream) -> TokenStream {
    // macro's attributes
    let mut attrs = Attributes::default();
    let arg_parser = meta::parser(|meta| attrs.parse(&meta));
    parse_macro_input!(args with arg_parser);

    // function the macro is decorating
    let item = parse_macro_input!(item as ItemFn);
    let async_fn = if item.sig.asyncness.is_some() {
        quote! { async }
    } else {
        quote! {}
    };

    // function signature
    let name = item.sig.ident.clone();
    let inputs = item.sig.inputs.clone();
    let output = item.sig.output.clone();
    let block = item.block;

    // macro attributes
    let span_name = attrs.name.unwrap_or_else(|| LitStr::new(&name.to_string(), name.span()));
    let level =
        attrs.level.map_or_else(|| quote! { ::tracing::Level::INFO }, |level| quote! {#level});

    // recreate function with the instrument macro wrapping it's block
    let new_fn = quote! {
        #[allow(non_snake_case)]
        #async_fn fn #name(#inputs) #output {
            let _guard = if tracing::Span::current().is_none() {
                let shutdown = ::sdk_otel::init();
                Some(shutdown)
            } else {
                None
            };
            tracing::span!(#level, #span_name).in_scope(|| {
                #block
            })
        }
    };

    TokenStream::from(new_fn)
}

#[derive(Default)]
struct Attributes {
    name: Option<LitStr>,
    level: Option<Expr>,
}

// See https://docs.rs/syn/latest/syn/meta/fn.parser.html
impl Attributes {
    fn parse(&mut self, meta: &ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("level") {
            self.level = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    }
}
