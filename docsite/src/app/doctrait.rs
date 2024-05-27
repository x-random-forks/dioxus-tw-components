use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_components::{atoms::ButtonVariant, attributes::{Animation, Color, Size}, form::{SelectGroup, SelectItem}};

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
        "Variation".to_string()
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

pub trait DemoComp {
    fn title() -> &'static str;
    fn preview_comp(demo_state: &DemoState) -> Element;
    fn select_attributes(demo_state: &DemoState) -> Element;
}

pub struct DemoState {
    custom_class: Signal<String>,
    override_class: Signal<String>,
    color: Signal<Color>,
    size: Signal<Size>,
    variant: Signal<ButtonVariant>,
    animation: Signal<Animation>,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            custom_class: use_signal(|| String::new()),
            override_class: use_signal(|| String::new()),
            color: use_signal(|| Color::default()),
            size: use_signal(|| Size::default()),
            variant: use_signal(|| ButtonVariant::default()),
            animation: use_signal(|| Animation::default()),
        }
    }

    pub fn get_custom_class(&self) -> Signal<String> {
        self.custom_class
    }

    pub fn get_override_class(&self) -> Signal<String> {
        self.override_class
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