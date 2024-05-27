use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn,
};

///
/// This procedural macro generates a new struct and function based on the input function.
/// The generated struct will have the same name as the input function with "Props" appended to it.
/// The struct will contain fields for each input parameter of the function and additional fields
/// specified in the macro arguments. The generated function will take an instance of the generated
/// struct as its only parameter and execute the same code as the input function. The goal was to make easier
/// the generation of component for the Dioxus web framework and to have some general attributes to be added
/// almost automatically and in the same way everywhere. For now we handle 3 attributes of this kind, these being
/// 1. id: *String* => used to give an id to the component
/// 2. class: *String* => used add custom style to the component, will add class and class_override to override the whole
/// class of the component
/// 3. children: *Element* => used to pass something to render as a child to the component \
/// **Example**
/// ```ignore
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
///     pub color: Color,
///     #[props(into)]
///     #[props(default = crate::hooks::use_unique_id())]
///     pub id: String,
///     #[props(into)]
///     #[props(default)]
///     pub class: String,
///     #[props(into, optional)]
///     pub class_override: String,
///     pub children: Element
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
    let mut vec_attr_props = Vec::new();

    for attr in &input.attrs {
        // Check if the attribute of the input(the function) is a doc comment
        if attr.path().is_ident("doc") {
            vec_attr.push(attr.clone());
        } else if attr.path().is_ident("derive") {
            vec_attr_props.push(attr.clone());
            // println!("Derive: {:?}", attr);
        }
    }

    let name = &input.sig.ident;
    let name_struct = syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());

    // Let statement are not used right now since they made the code far more complex to resolve problems about borrowing and such
    // any_mut is used to check if any of the parameters of the function is mutable, if so we will make the props mutable
    let (mut fields, mut let_statements, any_mut) =
        make_struct_fields_and_let_statements(&input.sig.inputs);

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

    let props = match any_mut {
        true => quote! { mut props: #name_struct },
        false => quote! {
            props: #name_struct
        },
    };

    let expanded = quote! {

        #[derive(Clone, Props, PartialEq)]
        #(#vec_attr_props)*
        pub struct #name_struct {
            #(#fields),*
        }

        #(#vec_attr)*
        pub fn #name(#props) #output {
            #(#let_statements)*

            let result = (|| #block)();
            result
        }
    };

    TokenStream::from(expanded)
}

/// This function takes a list of function parameters and generates a list of struct fields and
/// let statements for each parameter. The generated struct fields and let statements are used in
/// the props_component macro to generate the new struct and function.
fn make_struct_fields_and_let_statements(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> (
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    bool,
) {
    let mut fields = Vec::new();
    let let_statements = Vec::new();
    let mut any_mut = false;

    for input in inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let ident = match &*pat_type.pat {
                syn::Pat::Ident(pat_ident) => {
                    if pat_ident.mutability.is_some() {
                        any_mut = true;
                    }
                    &pat_ident.ident
                }
                _ => panic!("Unsupported parameter pattern"),
            };

            let ty = &pat_type.ty;
            let attr = &pat_type.attrs;

            fields.push(quote! { #(#attr)*pub #ident: #ty });
            // let_statements.push(quote! { let #ident = &props.#ident; });
        }
    }

    (fields, let_statements, any_mut)
}

/// This struct represents a list of identifiers parsed from the arguments of the props_component
/// macro. The identifiers specify additional fields to be added to the generated struct.
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

/// This struct represents a configuration of accepted attributes that can be added to the
/// generated struct in the props_component macro. Each accepted attribute has a name, a struct
/// field, and a let statement associated with it.
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

/// This struct represents an accepted attribute that can be added to the generated struct in the
/// props_component macro. It contains the name of the attribute, the struct field representing the
/// attribute, and the let statement used to destructure the attribute from the props parameter in
/// the generated function.
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
            field: quote! {
            /// Unique ID of the component
            #[props(into)] #[props(default = crate::hooks::use_unique_id())]pub id: String },
            let_statement: quote! { let id = &props.id; },
        }
    }

    fn class() -> Self {
        AcceptedAttribute {
            name: "class".to_string(),
            field: quote! {
            /// Custom added styling class for the component
            #[props(into)] #[props(default)]pub class: String,
            #[props(into, optional)] pub override_class: String },
            let_statement: quote! { let class = &props.class; },
        }
    }

    fn children() -> Self {
        AcceptedAttribute {
            name: "children".to_string(),
            field: quote! {
            /// Children of the component to render
            pub children: Element },
            let_statement: quote! { let children = &props.children; },
        }
    }
}
