use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Row, attributes(row))]
pub fn derive_row(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_row_derive(ast)
}

fn impl_row_derive(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let struct_generics = input.generics;

    let syn::Data::Struct(data) = input.data else {
        return quote::quote! {
            compile_error!("Only structs are supported");
        }
        .into();
    };

    let syn::Fields::Named(syn::FieldsNamed { named, .. }) = data.fields else {
        return quote::quote! {
            compile_error!("Only named fields supported");
        }
        .into();
    };

    let field_names: Vec<_> = named.iter().map(|field| field.ident.clone()).collect();
    // Example:
    // #[row(header="username")]
    // pub login: String
    let field_attrs: Vec<_> = named.iter().map(|field| field.attrs.clone()).collect();

    let field_types: Vec<_> = named.iter().map(|field| field.ty.clone()).collect();

    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    let field_column_names: Vec<_> = field_names
        .iter()
        .zip(field_attrs.iter())
        .map(|(name, attrs)| {
            let mut row_attr = RowAttr {
                header: None,
                sort: None,
                nosort: false,
            };

            for attr in attrs {
                if attr.path().is_ident("row") {
                    match attr.parse_args::<RowAttr>() {
                        Ok(parsed) => row_attr = parsed,
                        Err(err) => return err.to_compile_error().into(),
                    }
                }
            }
            match row_attr.header {
                Some(header) => {
                    quote::quote! {
                        #header.to_string()
                    }
                }
                None => {
                    quote::quote! {
                        stringify!(#name).to_string()
                    }
                }
            }
        })
        .collect();

    let fields_into_keytype: Vec<_> = field_names
        .iter()
        .zip(field_types.iter())
        .zip(field_attrs.iter())
        .map(|((name, mtype), attrs)| {
            let mut row_attr = RowAttr {
                header: None,
                sort: None,
                nosort: false,
            };

            for attr in attrs {
                if attr.path().is_ident("row") {
                    match attr.parse_args::<RowAttr>() {
                        Ok(parsed) => row_attr = parsed,
                        Err(err) => return err.to_compile_error().into(),
                    }
                }
            }

            let field_type = mtype.to_token_stream().to_string();
            let nosort = quote::format_ident!("{}", !row_attr.nosort);
            let element_sort = quote::format_ident!("{}", false);

            match field_type.as_str() {
                "Element" => {
                    quote::quote! {
                        KeyType::Element(self.#name.clone(), #element_sort)
                    }
                }
                "String" | "& str" | "& 'static str" => {
                    quote::quote! {
                        KeyType::String(self.#name.to_string(), #nosort)
                    }
                }
                "i8" | "i16" | "i32" | "i64" | "i128" => {
                    quote::quote! {
                        KeyType::Integer(self.#name.clone().into(), #nosort)
                    }
                }
                "u8" | "u16" | "u32" | "u64" | "u128" => {
                    quote::quote! {
                        KeyType::UnsignedInteger(self.#name.clone().into(), #nosort)
                    }
                }
                _ => quote::quote! {
                    KeyType::Object(Box::new(self.#name.clone()), #nosort)
                },
            }
        })
        .collect();

    let field_struct_sort: Vec<_> = field_names
        .iter()
        .zip(field_types.iter())
        .zip(field_attrs.iter())
        .map(|((_name, mtype), attrs)| {
            let mut row_attr = RowAttr {
                header: None,
                sort: None,
                nosort: false,
            };

            for attr in attrs {
                if attr.path().is_ident("row") {
                    match attr.parse_args::<RowAttr>() {
                        Ok(parsed) => row_attr = parsed,
                        Err(err) => return err.to_compile_error().into(),
                    }
                }
            }
            let field_type = mtype.to_token_stream().to_string();
            match field_type.as_str() {
                "String" | "& str" | "& 'static str" | "i8" | "i16" | "i32" | "i64" | "i128"
                | "u8" | "u16" | "u32" | "u64" | "u128" | "Element" => {
                    quote::quote! {}
                }
                _ => match row_attr.sort {
                    Some(sort) => {
                        let sort = quote::format_ident!("{}", sort);
                        quote::quote! {
                            impl Sortable for #mtype {
                                fn to_sortable(&self) -> KeyType {
                                    self.#sort.into()
                                }
                            }
                        }
                    }
                    None => {
                        quote::quote! {
                            impl Sortable for #mtype {}
                        }
                    }
                },
            }
        })
        .collect();

    quote! {
        #(#field_struct_sort)*
        impl #impl_generics ToTableData for #struct_name #ty_generics #where_clause {
            fn headers_to_strings() -> Vec<impl ToString> {
                vec![
                    #(
                        #field_column_names
                    ),*
                ]
            }

            fn to_keytype(&self) -> Vec<KeyType> {
                vec![
                    #(
                        #fields_into_keytype
                    ),*
                ]
            }
        }
    }
    .into()
}
// Define struct for `#[row(...)]` attributes
#[derive(Debug)]
struct RowAttr {
    header: Option<String>,
    sort: Option<String>,
    nosort: bool,
}

impl syn::parse::Parse for RowAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut header = None;
        let mut sort = None;
        let mut nosort = false;

        let args =
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;

        for meta in args.iter() {
            match meta {
                syn::Meta::NameValue(nv) if nv.path.is_ident("header") => {
                    header = Some(nv.value.to_token_stream().to_string().replace("\"", ""));
                }
                syn::Meta::NameValue(nv) if nv.path.is_ident("sort") => {
                    sort = Some(nv.value.to_token_stream().to_string().replace("\"", ""));
                }
                syn::Meta::Path(path) if path.is_ident("nosort") => {
                    nosort = true;
                }
                _ => return Err(syn::Error::new_spanned(meta, "Unexpected attribute")),
            }
        }

        if sort.is_some() && nosort {
            return Err(syn::Error::new_spanned(
                args,
                "Cannot use both `sort` and `nosort`",
            ));
        }

        Ok(RowAttr {
            header,
            sort,
            nosort,
        })
    }
}
