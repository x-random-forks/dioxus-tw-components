use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Generics, Ident};

pub fn impl_my_derive(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let struct_generic = &ast.generics;

    let struct_fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        fields
    } else {
        panic!("UiComp can only be derived on structs");
    };

    let derived_traits = derive_traits(struct_name, struct_fields, struct_generic);

    let gen = quote!(
        #(#derived_traits)*
    );
    gen.into()
}

fn derive_traits(
    struct_name: &Ident,
    fields: &Fields,
    struct_generics: &Generics,
) -> Vec<proc_macro2::TokenStream> {
    let buildclass_trait = build_buildclass_trait(struct_name, fields, struct_generics);

    let haschildren_trait = build_haschildren_trait(struct_name, fields, struct_generics);

    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();
    let uicomp_trait = quote! {
        impl #impl_generics UiComp for #struct_name #ty_generics #where_clause {}
    };

    // Get the name without "Props" to impl Display with
    let name = struct_name.to_string().replace("Props", "");

    let display_trait = quote! {
        impl #impl_generics std::fmt::Display for #struct_name #ty_generics #where_clause {
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

fn build_buildclass_trait(
    struct_name: &Ident,
    fields: &Fields,
    struct_generics: &Generics,
) -> proc_macro2::TokenStream {
    // Search for "attributes" in the fields of the struct
    let search_attributes = fields.iter().find(|field| {
        if let Some(ident) = &field.ident {
            *ident == "attributes"
        } else {
            false
        }
    });
    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    // If "attributes" is found return the vec as mut in BuildClass Trait
    if search_attributes.is_some() {
        quote! {
            impl #impl_generics BuildClass for #struct_name #ty_generics #where_clause {
                fn get_attributes(&mut self) -> Option<&mut Vec<Attribute>> {
                    Some(&mut self.attributes)
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics BuildClass for #struct_name #ty_generics #where_clause {}
        }
    }
}

fn build_haschildren_trait(
    struct_name: &Ident,
    fields: &Fields,
    struct_generics: &Generics,
) -> proc_macro2::TokenStream {
    // Search for "children" in the fields of the struct
    let search_attributes = fields.iter().find(|field| {
        if let Some(ident) = &field.ident {
            *ident == "children"
        } else {
            false
        }
    });
    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    // impl has_children as true and set_children accordingly else let the default definition of both
    if search_attributes.is_some() {
        quote! {
            impl #impl_generics HasChildren for #struct_name #ty_generics #where_clause {
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
            impl #impl_generics HasChildren for #struct_name #ty_generics #where_clause {}
        }
    }
}
