extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let props_name = &input.ident;

    let component_name = syn::Ident::new(
        &props_name.to_string().strip_suffix("Props").unwrap(),
        props_name.span(),
    );

    quote! {
        pub fn #component_name(props: #props_name) -> Element {
            props.view()
        }
    }
    .into()
}
