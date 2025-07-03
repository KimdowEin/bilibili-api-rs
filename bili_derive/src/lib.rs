extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(QueryTag, attributes(tag))]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let auth = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("tag"))
        .unwrap()
        .parse_args::<Ident>()
        .unwrap();
        
    let expanded = quote! {
        impl QueryTag for #name{
            const AUTH:AuthType = AuthType::#auth;
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Data)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Data for #name {}
    };
    TokenStream::from(expanded)
}
