use dioxus::prelude::*;
use dioxus_components::{
    atoms::ButtonVariant,
    attributes::{Animation, Color, Size},
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

pub trait DemoComponent {
    fn title() -> &'static str;
    fn description() -> &'static str;
    fn build_comp_preview() -> Element;
    fn build_comp_selectors() -> Element;
}
