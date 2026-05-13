use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use syn::{DeriveInput, parse_macro_input};

mod get;
mod helpers;
mod put;

#[proc_macro_error]
#[proc_macro_derive(StructfulGet, attributes(structful))]
pub fn derive_structful_get(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get::derive(input).into()
}

#[proc_macro_error]
#[proc_macro_derive(StructfulPut, attributes(structful))]
pub fn derive_structful_put(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    put::derive(input).into()
}
