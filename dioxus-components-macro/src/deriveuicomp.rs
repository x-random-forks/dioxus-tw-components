use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident};

pub fn impl_my_derive(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let struct_fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        fields
    } else {
        panic!("UiComp can only be derived on structs");
    };

    let derived_traits = derive_traits(struct_name, struct_fields);

    let gen = quote!(
        #(#derived_traits)*
    );
    gen.into()
}

fn derive_traits(struct_name: &Ident, fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let buildclass_trait = build_buildclass_trait(struct_name, fields);

    let haschildren_trait = build_haschildren_trait(struct_name, fields);

    let uicomp_trait = quote! {
        impl UiComp for #struct_name {}
    };

    // Get the name without "Props" to impl Display with
    let name = struct_name.to_string().replace("Props", "");

    let display_trait = quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(#name)
            }
        }
    };

    vec![
        buildclass_trait,
        haschildren_trait,
        uicomp_trait,
        display_trait,
    ]
}

fn build_buildclass_trait(struct_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    // Search for "attributes" in the fields of the struct
    let search_attributes = fields.iter().find(|field| {
        if let Some(ident) = &field.ident {
            ident.to_string() == "attributes"
        } else {
            false
        }
    });

    // If "attributes" is found return the vec as mut in BuildClass Trait
    if let Some(_) = search_attributes {
        quote! {
            impl BuildClass for #struct_name {
                fn get_attributes(&mut self) -> Option<&mut Vec<Attribute>> {
                    Some(&mut self.attributes)
                }
            }
        }
    } else {
        quote! {
            impl BuildClass for #struct_name {}
        }
    }
}

fn build_haschildren_trait(struct_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    // Search for "children" in the fields of the struct
    let search_attributes = fields.iter().find(|field| {
        if let Some(ident) = &field.ident {
            ident.to_string() == "children"
        } else {
            false
        }
    });

    // impl has_children as true and set_children accordingly else let the default definition of both
    if let Some(_) = search_attributes {
        quote! {
            impl HasChildren for #struct_name {
                fn has_children(&self) -> bool {
                    true
                }

                fn set_children(&mut self, children: Element) {
                    self.children = children;
                }
            }
        }
    } else {
        quote! {
            impl HasChildren for #struct_name {}
        }
    }
    .into()
}
