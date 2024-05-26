use std::{fmt::Display, str::FromStr};

use dioxus::{dioxus_core::DynamicNode, prelude::*};
use dioxus_components::{
    atoms::{button::ButtonProps, Button, ButtonVariant},
    attributes::{Color, Size},
    form::{SelectGroup, SelectItem},
};

#[component]
pub fn ButtonPage() -> Element {
    rsx!(
        h1 { class: "h1", "Button" }
        p { class: "paragraph", "A simple yet customizable button" }
        p { class: "paragraph anchor", "Source should be outlink" }
        p { class: "paragraph anchor", "Page Source should outlink" }
        demo_preview {}
    )
}

pub trait IntoVec<T> {
    fn into_vec() -> Vec<T>;
}

impl IntoVec<Color> for Color {
    fn into_vec() -> Vec<Color> {
        vec![
            Color::Default,
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Destructive,
            Color::Accent,
            Color::Muted,
        ]
    }
}

impl IntoVec<Size> for Size {
    fn into_vec() -> Vec<Size> {
        vec![Size::Md, Size::Xs, Size::Sm, Size::Lg, Size::Xl]
    }
}

impl IntoVec<ButtonVariant> for ButtonVariant {
    fn into_vec() -> Vec<ButtonVariant> {
        vec![
            ButtonVariant::Default,
            ButtonVariant::Outline,
            ButtonVariant::Ghost,
        ]
    }
}

pub trait DemoAttribute<T>: PartialEq + FromStr + Default + Display + IntoVec<T> + 'static {}

impl<T> DemoAttribute<T> for Color where Color: IntoVec<T> {}

impl<T> DemoAttribute<T> for Size where Size: IntoVec<T> {}

impl<T> DemoAttribute<T> for ButtonVariant where ButtonVariant: IntoVec<T> {}

trait DemoComp {
    fn view() -> Element;
}

impl DemoComp for ButtonProps {
    fn view() -> Element {
        rsx!(
            Button {"VIEW"}
        )
    }
}

#[component]
fn demo_attribute_test<T: DemoAttribute<T>>(signal: Signal<T>) -> Element {
    rsx!(
        SelectGroup {
            oninput: move |event: FormEvent| {
                signal.set(T::from_str(&event.data().value()).unwrap_or_default())
            },
            for item in T::into_vec() {
                SelectItem { value: item.to_string().to_lowercase(), {item.to_string()} }
            }
        }
    )
}

#[component]
fn demo_preview() -> Element {
    let color = use_signal(|| Color::default());
    let size = use_signal(|| Size::default());
    let variant = use_signal(|| ButtonVariant::default());

    let _b = ButtonProps::builder()
        .children(rsx!("abc"))
        .color(color())
        .size(size())
        .variant(variant())
        .build();

    let test = _b.into_vcomponent(Button, "");

    let test2 = DynamicNode::Component(test);

    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", {  } }
            div { class: "border border-yellow-500", "header" }
            div { class: "flex items-center justify-center", PreviewTemplate {
                {test2}
                {ButtonProps::view()}
                Button { color: color(), size: size(), variant: variant(), "Button" }
            } }
            div { class: "border border-red-600", "footer" }
            demo_attribute_test { signal: color }
            demo_attribute_test { signal: size }
            demo_attribute_test { signal: variant }
        }
    )
}

#[component]
pub fn DemoTemplate(#[props(optional)] title: String, children: Element) -> Element {
    let color = use_signal(|| Color::default());
    let size = use_signal(|| Size::default());

    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", { title } }
            div { class: "border border-yellow-500", "header" }
            div { class: "flex items-center justify-center", PreviewTemplate {
            } }
            div { class: "border border-red-600", "footer" }
            // {color.render()},
            // {Color::demo(color)},
            demo_attribute_test { signal: color }
            demo_attribute_test { signal: size }

        }
    )
}

#[component]
pub fn PreviewTemplate(children: Element) -> Element {
    rsx!(
        div { class: "p-4", { children } }
    )
}
