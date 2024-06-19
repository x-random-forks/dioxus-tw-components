use dioxus::prelude::*;
use dioxus_components::{
    atoms::ButtonVariant,
    attributes::{Animation, Color, Orientation, Size},
};

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

impl IntoVec<Orientation> for Orientation {
    fn into_vec() -> Vec<Orientation> {
        vec![Orientation::Vertical, Orientation::Horizontal]
    }
}

pub trait DemoComponent {
    fn title() -> &'static str;
    fn description() -> &'static str;
    fn BuildCompPreview() -> Element;
    fn BuildCompSelectors() -> Element;
}
