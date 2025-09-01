use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Error, Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::{Result, braced, bracketed, token};

// use crate::parse::{Json, Value};

// Create strongly typed `CreateOfferRequest` from JSON-like input,
// doing basic validation and setting sensible defaults.
pub fn generate(input: &Json) -> Result<TokenStream> {
    let span = Span::call_site();

    let mut input = input.clone();

    let credential_issuer = input.expect("credential_issuer")?;
    let subject_id = input.option("subject_id");

    // one or more `credential_configuration_ids` are required
    let Some(credential_configuration_ids) = input.get("credential_configuration_ids") else {
        return Err(Error::new(span, "`credential_configuration_ids` is not set"));
    };
    let Some(ids) = &credential_configuration_ids.as_array() else {
        return Err(Error::new(span, "`credential_configuration_ids` must be an array"));
    };
    if ids.is_empty() {
        return Err(Error::new(span, "`credential_configuration_ids` cannot be empty"));
    }

    // use default values if not set
    let pre_authorize = input.get("pre_authorize").unwrap_or(Value::Bool(false));
    let tx_code_required = input.get("tx_code_required").unwrap_or(Value::Bool(false));
    let send_type =
        input.get("send_type").map_or_else(|| quote! {SendType::ByVal}, |v| quote! {#v.into()});

    // return error for any unexpected fields
    input.check_consumed()?;

    let path = quote! {vercre_issuer};

    Ok(quote! {
        #path::CreateOfferRequest {
            credential_issuer: #credential_issuer,
            subject_id: #subject_id,
            credential_configuration_ids: #credential_configuration_ids,
            pre_authorize: #pre_authorize,
            tx_code_required: #tx_code_required,
            send_type:  #send_type,
        }
    })
}

#[derive(Default, Clone)]
pub struct Json {
    pub fields: HashMap<String, Value>,
}

impl Json {
    /// Get the value for the key, removing it from the JSON object.
    pub fn get(&mut self, key: &str) -> Option<Value> {
        self.fields.remove(key)
    }

    /// Expect the key to be present and return the value or an error.
    pub fn expect(&mut self, key: &str) -> Result<Value> {
        let Some(v) = self.fields.remove(key) else {
            return Err(Error::new(Span::call_site(), format!("`{key}` is not set")));
        };
        Ok(Value::Tokens(quote! {Into::into(#v)}))
    }

    /// Either `Some` or `None` depending on whether the key is present.
    pub fn option(&mut self, key: &str) -> Value {
        self.fields.remove(key).map_or_else(
            || Value::Tokens(quote! {None}),
            |v: Value| Value::Tokens(quote! {Some(#v.into())}),
        )
    }

    /// Check all parsed fields have been consumed, returning an error if any
    /// fields are left unconsumed.
    pub fn check_consumed(&self) -> Result<()> {
        if !self.fields.is_empty() {
            let keys = self.fields.keys().map(|k| format!("`{k}`")).collect::<Vec<_>>().join(", ");
            return Err(Error::new(Span::call_site(), format!("unexpected field(s): {keys}")));
        }
        Ok(())
    }
}

struct Field {
    lhs: String,
    rhs: Value,
}

#[derive(Debug, Clone, Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Self>),
    Object(HashMap<String, Self>),
    Ident(syn::Ident),
    Tokens(TokenStream),
}

// Parse the macro contents into a Data struct.
impl Parse for Json {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut json = Self::default();

        // content should be wrapped in braces
        if !input.peek(token::Brace) {
            return Err(input.error("expected JSON object"));
        }

        let content;
        braced!(content in input);

        // parse key-value pairs into fields
        let fields = Punctuated::<Field, token::Comma>::parse_terminated(&content)?;
        for field in fields.into_pairs() {
            let field = field.into_value();
            json.fields.insert(field.lhs, field.rhs);
        }

        Ok(json)
    }
}

// Parse key-value pair into a Field.
impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let lhs = input.parse::<syn::LitStr>()?;
        // chomp colon
        input.parse::<token::Colon>()?;

        Ok(Self {
            lhs: lhs.value(),
            rhs: input.parse::<Value>()?,
        })
    }
}

// Parse the RHS of key-value pair into a Value.
impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();

        let rhs = if l.peek(syn::LitStr) {
            let string = input.parse::<syn::LitStr>()?.value();
            Self::String(string)
        } else if l.peek(syn::LitInt) {
            Self::Number(input.parse::<syn::LitInt>()?.base10_parse::<i64>()?)
        } else if l.peek(syn::LitBool) {
            Self::Bool(input.parse::<syn::LitBool>()?.value())
        } else if l.peek(token::Brace) {
            // parse object
            let mut data = HashMap::new();
            let content;
            braced!(content in input);
            let fields = Punctuated::<Field, token::Comma>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                let field = field.into_value();
                data.insert(field.lhs, field.rhs);
            }
            Self::Object(data)
        } else if l.peek(token::Bracket) {
            // parse array
            let contents;
            bracketed!(contents in input);
            let items = Punctuated::<Self, token::Comma>::parse_terminated(&contents)?;
            let values = items.into_pairs().map(Pair::into_value).collect();
            Self::Array(values)
        } else if l.peek(syn::Ident)
            && (input.peek2(token::PathSep) || input.peek2(token::Paren) || input.peek2(token::Dot))
        {
            // parse enum variant or method call
            Self::Tokens(input.parse::<syn::Expr>()?.to_token_stream())
        } else if l.peek(syn::Ident) {
            // parse const or variable
            Self::Ident(input.parse::<syn::Ident>()?)
        } else if l.peek(token::And) {
            // chomp `&` and re-parse
            input.parse::<token::And>()?;
            input.parse::<Self>()?
        } else {
            return Err(input.error("unexpected token"));
        };

        Ok(rhs)
    }
}

// Implement ToTokens for Value to generate the corresponding Rust code.
impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Null => tokens.extend(quote! { None }),
            Self::Bool(b) => tokens.extend(quote! { #b }),
            Self::String(s) => tokens.extend(quote! { #s.to_string() }),
            Self::Number(n) => tokens.extend(quote! { #n }),
            Self::Ident(i) => tokens.extend(quote! { #i }),
            Self::Tokens(t) => tokens.extend(t.clone()),
            Self::Array(a) => {
                let values = a.iter().map(|v| {
                    let mut tokens = TokenStream::new();
                    v.to_tokens(&mut tokens);
                    tokens
                });
                tokens.extend(quote! { vec![#(#values),*] });
            }
            Self::Object(_) => {
                unimplemented!("Value::to_tokens for Object");
                //     let fields = o.iter().map(|(k, v)| {
                //         let mut tokens = TokenStream::new();
                //         v.to_tokens(&mut tokens);
                //         quote! {#k: #tokens}

                //         // quote! {#k: #v}
                //     });
                //     tokens.extend(quote! { #(#fields),* });
            }
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub const fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    pub const fn as_object(&self) -> Option<&HashMap<String, Self>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}

// Dump out remainder of input stream for debugging
#[allow(dead_code)]
fn dump(input: ParseStream) -> Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            println!("{tt:?}");
            rest = next;
        }
        Ok(((), rest))
    })
}
