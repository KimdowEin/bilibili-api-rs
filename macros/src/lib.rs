extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Query for #name{}
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Sign)]
pub fn derive_sign(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Sign for #name {}
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Csrf)]
pub fn derive_csrf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Csrf for #name {}
    };
    TokenStream::from(expanded)
}
