use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse::Parser, parse_macro_input, FieldsNamed, ItemStruct};

#[proc_macro_attribute]
pub fn dxcomp(args: TokenStream, input: TokenStream) -> TokenStream {
    const ACCEPTED_ATTRIBUTES: [&str; 2] = ["color", "size"];

    let mut input = parse_macro_input!(input as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = input.fields {
        add_field_to_struct(fields, quote! { children: Element }, None);

        add_field_to_struct(
            fields,
            quote! { id: String },
            Some(quote! { #[props(default)] }),
        );

        add_field_to_struct(
            fields,
            quote! { class: String },
            Some(quote! { #[props(default)] }),
        );

        // handle_macro_args();
        for arg in args {
            if let TokenTree::Ident(ident) = arg {
                if !ACCEPTED_ATTRIBUTES.contains(&ident.to_string().as_str()) {
                    panic!("Invalid attribute: {}", ident);
                } else {
                    match ident.to_string().as_str() {
                        "color" => {
                            add_field_to_struct(
                                fields,
                                quote! { pub(crate) color: Color },
                                Some(quote! { #[props(default = Color::Primary)] }),
                            );
                        }
                        "size" => {
                            add_field_to_struct(
                                fields,
                                quote! { size: Size },
                                Some(quote! { #[props(default = Size::Medium)] }),
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    print_pretty_tokens(&input);

    return quote! {
        #input
    }
    .into();
}

fn add_field_to_struct(
    fields: &mut FieldsNamed,
    field: proc_macro2::TokenStream,
    attrs: Option<proc_macro2::TokenStream>,
) {
    let mut new_field = syn::Field::parse_named.parse2(field.into()).unwrap();

    if let Some(attrs) = attrs {
        let attrs = syn::Attribute::parse_outer.parse2(attrs.into()).unwrap();
        new_field.attrs.extend(attrs);
    }

    fields.named.push(new_field);
}

fn print_pretty_tokens(input: &ItemStruct) {
    let pretty_tokens = quote! {
        #input
    };

    println!("pretty_tokens: \"{}\"", pretty_tokens);
}
