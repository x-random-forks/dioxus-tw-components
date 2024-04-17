use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn,
};

/// This macro is used to create a component that takes props as arguments, only 3 attributes are handled for now : id, class and children
/// ```
/// /// This is a cool and very useful component
/// #[props_component(id, class, children)]
/// pub fn NewElement(
/// /// This is to color the component
/// #[props(default = Color::Destructive)] color: Color
/// ) -> Element
///
/// // Will expand to this struct
///
/// #[derive(Clone, Props, PartialEq)]
/// pub struct NewElementProps {
///     /// This is to color this cool component
///     #[props(default = Color::Destructive)]
///     color: Color,
///     #[props(into)]
///     #[props(default)]
///     id: String,
///     #[props(into)]
///     #[props(default)]
///     class: String,
///     children: Element
/// }
///
/// // and the function signature will be changed to this
///
/// /// This is a cool and very useful component
/// pub fn NewElement(props: NewElementProps) -> Element
/// ```
///
#[proc_macro_attribute]
pub fn props_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let mut vec_attr = Vec::new();

    for attr in &input.attrs {
        // Check if the attribute is a doc comment
        if attr.path().is_ident("doc") {
            vec_attr.push(attr.clone());
        }
    }

    // println!("'{:?}' {}", vec_attr, vec_attr.len());

    let name = &input.sig.ident;
    let name_struct = syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());

    // Let statement are not used right now since they made the code far more complex to resolve problems about borrowing and such
    let (mut fields, mut let_statements) = make_struct_fields_and_let_statements(&input.sig.inputs);

    let args = parse_macro_input!(args as ParsedArg);
    let accepted_attributes = AttributeConfig::default();

    for arg in args.args {
        match accepted_attributes.accepted(arg.to_string()) {
            Ok(index) => {
                fields.push(accepted_attributes.add_attributes[index].field.clone());
                let_statements.push(
                    accepted_attributes.add_attributes[index]
                        .let_statement
                        .clone(),
                )
            }
            Err(e) => panic!("{e}"),
        }
    }

    let output = &input.sig.output;
    let block = &input.block;

    let expanded = quote! {

        #[derive(Clone, Props, PartialEq)]
        pub struct #name_struct {
            #(#fields),*
        }

        #(#vec_attr)*
        pub fn #name(props: #name_struct) #output {
            #(#let_statements)*

            let result = (|| #block)();
            result
        }
    };

    TokenStream::from(expanded)
}

fn make_struct_fields_and_let_statements(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut fields = Vec::new();
    let let_statements = Vec::new();

    for input in inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let ident = match &*pat_type.pat {
                syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => panic!("Unsupported parameter pattern"),
            };

            let ty = &pat_type.ty;
            let attr = &pat_type.attrs;

            fields.push(quote! { #(#attr)*pub #ident: #ty });
            // let_statements.push(quote! { let #ident = &props.#ident; });
        }
    }

    (fields, let_statements)
}

#[derive(Debug)]
struct ParsedArg {
    args: Vec<syn::Ident>,
}

impl Parse for ParsedArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args =
            syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated(&input)?;

        Ok(ParsedArg {
            args: args.into_iter().collect(),
        })
    }
}

#[derive(Debug)]
struct AttributeConfig {
    add_attributes: Vec<AcceptedAttribute>,
}

impl AttributeConfig {
    fn default() -> Self {
        let mut add_attributes = Vec::new();

        add_attributes.push(AcceptedAttribute::id());
        add_attributes.push(AcceptedAttribute::class());
        add_attributes.push(AcceptedAttribute::children());

        AttributeConfig { add_attributes }
    }

    fn accepted(&self, attr_name: String) -> Result<usize, String> {
        for (index, attribute) in self.add_attributes.iter().enumerate() {
            if attribute.name == attr_name {
                return Ok(index);
            }
        }

        Err(format!("Invalid attribute: {}", attr_name))
    }
}

#[derive(Debug, Clone)]
struct AcceptedAttribute {
    name: String,
    field: proc_macro2::TokenStream,
    let_statement: proc_macro2::TokenStream,
}

impl AcceptedAttribute {
    fn id() -> Self {
        AcceptedAttribute {
            name: "id".to_string(),
            field: quote! { #[props(into)] #[props(default = crate::hooks::use_unique_id())]id: String },
            let_statement: quote! { let id = &props.id; },
        }
    }

    fn class() -> Self {
        AcceptedAttribute {
            name: "class".to_string(),
            field: quote! { #[props(into)] #[props(default)]class: String },
            let_statement: quote! { let class = &props.class; },
        }
    }

    fn children() -> Self {
        AcceptedAttribute {
            name: "children".to_string(),
            field: quote! { children: Element },
            let_statement: quote! { let children = &props.children; },
        }
    }
}
