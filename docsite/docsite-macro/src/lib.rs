use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use syn::{parse2, parse_macro_input, Ident, Stmt};

/// This is used for docsite to generate each component declaration
#[proc_macro_attribute]
pub fn preview_comp_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let comp_name = syn::parse_macro_input!(attr as syn::Ident);
    println!("Struct {:?}", comp_name);

    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let mut block = input.block.clone();

    let name = &input.sig.ident;

    let res = quote! {};
    let stmt: Stmt = parse2(res.into()).unwrap();

    block.stmts.insert(0, stmt);

    TokenStream::from(input.into_token_stream())
}
