use dioxus::{html::geometry::*, prelude::IntoAttributeValue};

pub trait BaseClass {
    fn base(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Success,
    Accent,
    Muted,
}

pub trait Colorable {
    fn color(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

pub trait Sizable {
    fn size(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

pub trait Orientable {
    fn orientation(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Animation {
    None,
    Light,
    #[default]
    Full,
    Custom(&'static str),
}

pub trait Animatable {
    fn animation(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Side {
    #[default]
    Left,
    Right,
}

pub trait Variation {
    fn variant(&self) -> &'static str;
}

// TODO use this everywhere we can
#[derive(Clone, Copy, Debug)]
pub enum StateAttribute {
    Active,
    Inactive,
}

impl StateAttribute {
    pub fn state_attribute_to_str(&self) -> &'static str {
        match self {
            StateAttribute::Active => "active",
            StateAttribute::Inactive => "inactive",
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            StateAttribute::Active => true,
            StateAttribute::Inactive => false,
        }
    }
}

impl IntoAttributeValue for StateAttribute {
    fn into_value(self) -> dioxus::prelude::dioxus_core::AttributeValue {
        dioxus::prelude::dioxus_core::AttributeValue::Text(
            self.state_attribute_to_str().to_string(),
        )
    }
}

pub struct AppState {
    last_click_coordinates: Coordinates,
}

impl AppState {
    pub fn default() -> Self {
        Self {
            last_click_coordinates: Coordinates::new(
                ScreenPoint::zero(),
                ClientPoint::zero(),
                ElementPoint::zero(),
                PagePoint::zero(),
            ),
        }
    }

    pub fn get_last_click_coordinates(&self) -> &Coordinates {
        &self.last_click_coordinates
    }

    pub fn set_last_click_coordinates(&mut self, coordinates: Coordinates) {
        self.last_click_coordinates = coordinates;
    }
}
