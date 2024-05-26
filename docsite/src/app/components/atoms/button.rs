use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_components::{
    atoms::{
        button::ButtonProps, placeholder::PlaceholderProps, Button, ButtonVariant, Placeholder,
    },
    attributes::{Animation, Color, Size},
    form::{Input, SelectGroup, SelectItem},
};

#[component]
pub fn ButtonPage() -> Element {
    rsx!(
        h1 { class: "h1", "Button" }
        p { class: "paragraph", "A simple yet customizable button" }
        p { class: "paragraph anchor", "Source should be outlink" }
        p { class: "paragraph anchor", "Page Source should outlink" }
        demo_preview::<ButtonProps> {}
        demo_preview::<PlaceholderProps> {}
    )
}

pub trait BaseNameToStr {
    fn base_name_to_str() -> String;
}

impl BaseNameToStr for Color {
    fn base_name_to_str() -> String {
        "Color".to_string()
    }
}

impl BaseNameToStr for Size {
    fn base_name_to_str() -> String {
        "Size".to_string()
    }
}

impl BaseNameToStr for ButtonVariant {
    fn base_name_to_str() -> String {
        "Button Variant".to_string()
    }
}

impl BaseNameToStr for Animation {
    fn base_name_to_str() -> String {
        "Animation".to_string()
    }
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

impl IntoVec<Animation> for Animation {
    fn into_vec() -> Vec<Animation> {
        vec![Animation::Full, Animation::None, Animation::Light]
    }
}

pub trait DemoAttribute<T>
where
    T: PartialEq + FromStr + Default + Display + IntoVec<T> + BaseNameToStr + 'static,
{
    fn demo_attr(mut signal: Signal<T>) -> Element {
        rsx!(
            div { class: "flex flex-col",
                p { class: "paragraph", {T::base_name_to_str()} }
                SelectGroup {
                    oninput: move |event: FormEvent| {
                        signal.set(T::from_str(&event.data().value()).unwrap_or_default())
                    },
                    for item in T::into_vec() {
                        SelectItem { value: item.to_string().to_lowercase(), {item.to_string()} }
                    }
                }
            }
        )
    }
}

impl<T> DemoAttribute<T> for Color
where
    Color: IntoVec<T>,
    T: PartialEq + FromStr + Default + Display + IntoVec<T> + BaseNameToStr + 'static,
{
}

impl<T> DemoAttribute<T> for Size
where
    Size: IntoVec<T>,
    T: PartialEq + FromStr + Default + Display + IntoVec<T> + BaseNameToStr + 'static,
{
}

impl<T> DemoAttribute<T> for ButtonVariant
where
    ButtonVariant: IntoVec<T>,
    T: PartialEq + FromStr + Default + Display + IntoVec<T> + BaseNameToStr + 'static,
{
}

impl<T> DemoAttribute<T> for Animation
where
    Animation: IntoVec<T>,
    T: PartialEq + FromStr + Default + Display + IntoVec<T> + BaseNameToStr + 'static,
{
}

trait DemoComp {
    fn view(demo_state: &DemoState) -> Element;
    fn view_attributes(demo_state: &DemoState) -> Element;
}

impl DemoComp for ButtonProps {
    fn view(demo_state: &DemoState) -> Element {
        rsx!(
            Button {
                color: demo_state.get_color()(),
                size: demo_state.get_size()(),
                variant: demo_state.get_variant()(),
                animation: demo_state.get_animation()(),
                class: demo_state.get_custom_class()(),
                "Button"
            }
        )
    }

    fn view_attributes(demo_state: &DemoState) -> Element {
        rsx!(
            demo_custom_class { signal: demo_state.get_custom_class() }
            demo_group_attr {
                {Color::demo_attr(demo_state.get_color())},
                {Size::demo_attr(demo_state.get_size())},
                {ButtonVariant::demo_attr(demo_state.get_variant())},
                {Animation::demo_attr(demo_state.get_animation())}
            }
        )
    }
}

#[component]
pub fn demo_group_attr(children: Element) -> Element {
    rsx!(
        div { class: "grid grid-flow-col w-full", { children } }
    )
}

impl DemoComp for PlaceholderProps {
    fn view(demo_state: &DemoState) -> Element {
        rsx!(Placeholder {
            animation: demo_state.get_animation()(),
            class: demo_state.get_custom_class()()
        })
    }

    fn view_attributes(demo_state: &DemoState) -> Element {
        rsx!(
            demo_custom_class { signal: demo_state.get_custom_class() }
            demo_group_attr { {Animation::demo_attr(demo_state.get_animation())} }
        )
    }
}

#[component]
fn demo_custom_class(signal: Signal<String>) -> Element {
    rsx!(
        div { class: "flex flex-row items-center gap-2",
            "Class"
            Input { oninput: move |event: FormEvent| { signal.set(event.data().value()) } }
        }
    )
}

struct DemoState {
    custom_class: Signal<String>,
    color: Signal<Color>,
    size: Signal<Size>,
    variant: Signal<ButtonVariant>,
    animation: Signal<Animation>,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            custom_class: use_signal(|| String::new()),
            color: use_signal(|| Color::default()),
            size: use_signal(|| Size::default()),
            variant: use_signal(|| ButtonVariant::default()),
            animation: use_signal(|| Animation::default()),
        }
    }

    pub fn get_custom_class(&self) -> Signal<String> {
        self.custom_class
    }

    pub fn get_color(&self) -> Signal<Color> {
        self.color
    }

    pub fn get_size(&self) -> Signal<Size> {
        self.size
    }

    pub fn get_variant(&self) -> Signal<ButtonVariant> {
        self.variant
    }

    pub fn get_animation(&self) -> Signal<Animation> {
        self.animation
    }
}

#[component]
fn demo_preview<T: DemoComp>() -> Element {
    let demo_state = DemoState::new();

    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", {  } }
            div { class: "border border-yellow-500", "header" }
            div { class: "flex items-center justify-center", PreviewTemplate { {T::view(&demo_state)} } }
            div { class: "border border-red-600", "footer" }
            {T::view_attributes(&demo_state)}
        }
    )
}

#[component]
pub fn PreviewTemplate(children: Element) -> Element {
    rsx!(
        div { class: "p-4 min-h-64 border border-black flex justify-center items-center",
            { children }
        }
    )
}