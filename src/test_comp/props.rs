// use dioxus::prelude::*;
// use props_component_macro::props_component;

// pub trait BaseClass {
//     fn base(&self) -> &'static str;
// }

// pub trait Colorable {
//     fn color(&self) -> &'static str;
// }

// #[derive(Default, Clone, Copy, PartialEq, Debug)]
// pub enum Color {
//     #[default]
//     Default,
//     Primary,
//     Secondary,
//     Destructive,
//     Success,
//     Accent,
//     Muted,
// }

// pub trait Sizable {
//     fn size(&self) -> &'static str;
// }

// pub trait Variation {
//     fn variant(&self) -> &'static str;
// }

// ////////////////////////////////////////////////////////////////////////////////////////

// // #[dxcomp]
// // #[derive(Clone, Props, PartialEq)]
// // pub struct TestTextProps {
// //     #[props(default = Color::Primary)]
// //     pub(crate) color: Color,
// //     #[props(default)]
// //     pub(crate) variant: TestTextVariant,
// // }

// // pub fn TestText(props: TestTextProps) -> Element {
// //     let class = tw_merge!(props.base(), props.color(), props.variant());

// //     // log::debug!("class: {:?}", class);

// //     rsx!( div { class: class, "Hello World" } )
// // }

// // ////////////////////////////////////////////////////////////////////////////////////////

// // #[derive(Clone, Props, PartialEq)]
// // pub struct TestBoxProps {
// //     #[props(default = Color::Destructive)]
// //     pub(crate) color: Color,
// //     #[props(default)]
// //     pub(crate) variant: TestBoxVariant,
// // }

// // pub fn TestBox(props: TestBoxProps) -> Element {
// //     let class = tw_merge!(props.base(), props.color(), props.variant());

// //     // log::debug!("class: {:?}", class);

// //     rsx!(div { class: class })
// // }

// ////////////////////////////////////////////////////////////////////////////////////////

// #[props_component(id, children, class)]
// pub fn NewElement(#[props(default = Color::Destructive)] color: Color) -> Element {
//     log::debug!("id: {:?}", id);
//     log::debug!("class: {:?}", class);
//     log::debug!("color: {:?}", color);

//     rsx!(
//         div { class: class, { children } }
//     )
// }
