use std::{ops::Neg, str::FromStr};

use dioxus::{dioxus_core::Element, prelude::IntoAttributeValue};

pub trait HasChildren {
    fn has_children(&self) -> bool {false}
    fn set_children(&mut self, _children: Element) {} 
}

pub trait Class {
    fn base(&self) -> &'static str {
        ""
    }

    fn color(&self) -> Option<&'static str> {
        None
    }

    fn size(&self) -> Option<&'static str> {
        None
    }

    fn variant(&self) -> Option<&'static str> {
        None
    }

    fn animation(&self) -> Option<&'static str> {
        None
    }

    fn orientation(&self) -> Option<&'static str> {
        None
    }
}

pub trait BuildClass: Class {
    fn build_class(&mut self);

    // All those below are only used for the docsite
    fn set_class(&mut self, class: String);

    fn set_override_class(&mut self, override_class: String);

    fn has_color(&self) -> bool {
        self.color().is_some()
    }

    fn set_color(&mut self, _color: Color) {}

    fn has_size(&self) -> bool {
        self.size().is_some()
    }

    fn set_size(&mut self, _size: Size) {}

    fn has_animation(&self) -> bool {
        self.animation().is_some()
    }

    fn set_animation(&mut self, _animation: Animation) {}

    fn has_orientation(&self) -> bool {
        self.orientation().is_some()
    }

    fn set_orientation(&mut self, _orientation: Orientation) {}
}

pub trait Named {
    const NAME: &'static str;

    fn name() -> &'static str {
        Self::NAME
    }
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

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" => Ok(Color::Primary),
            "secondary" => Ok(Color::Secondary),
            "destructive" => Ok(Color::Destructive),
            "success" => Ok(Color::Success),
            "accent" => Ok(Color::Accent),
            "muted" => Ok(Color::Muted),
            "default" | _ => Ok(Color::default()),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::Default => "Default",
            Color::Primary => "Primary",
            Color::Secondary => "Secondary",
            Color::Destructive => "Destructive",
            Color::Success => "Success",
            Color::Accent => "Accent",
            Color::Muted => "Muted",
        };
        f.write_str(s)
    }
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

impl FromStr for Size {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "xs" => Ok(Size::Xs),
            "sm" => Ok(Size::Sm),
            "md" => Ok(Size::Md),
            "lg" => Ok(Size::Lg),
            "xl" => Ok(Size::Xl),
            "default" | _ => Ok(Size::default()),
        }
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Size::Xs => "Xs",
            Size::Sm => "Sm",
            Size::Md => "Md",
            Size::Lg => "Lg",
            Size::Xl => "Xl",
        };
        f.write_str(s)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "horizontal" => Ok(Orientation::Horizontal),
            _ => Ok(Orientation::Vertical),
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Orientation::Vertical => "vertical",
            Orientation::Horizontal => "horizontal"
        };
        f.write_str(s)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Animation {
    None,
    Light,
    #[default]
    Full,
    Custom(&'static str),
}

impl FromStr for Animation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Animation::None),
            "light" => Ok(Animation::Light),
            "custom" => Ok(Animation::Custom("")),
            "full" | _ => Ok(Animation::Full),
        }
    }
}

impl std::fmt::Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Animation::None => "None",
            Animation::Light => "Light",
            Animation::Full => "Full",
            Animation::Custom(_str) => "Custom",
        };
        f.write_str(s)
    }
}

// TODO use this everywhere we can
#[derive(Clone, Copy, Debug)]
pub enum DataStateAttrValue {
    Active,
    Inactive,
}

impl DataStateAttrValue {
    pub fn state_attribute_to_str(&self) -> &'static str {
        match self {
            DataStateAttrValue::Active => "active",
            DataStateAttrValue::Inactive => "inactive",
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            DataStateAttrValue::Active => true,
            DataStateAttrValue::Inactive => false,
        }
    }

    pub fn toggle(&mut self) {
        *self = -(*self);
    }
}

impl From<DataStateAttrValue> for &'static str {
    fn from(value: DataStateAttrValue) -> Self {
        match value {
            DataStateAttrValue::Active => "active",
            DataStateAttrValue::Inactive => "inactive",
        }
    }
}

impl IntoAttributeValue for DataStateAttrValue {
    fn into_value(self) -> dioxus::prelude::dioxus_core::AttributeValue {
        dioxus::prelude::dioxus_core::AttributeValue::Text(
            self.state_attribute_to_str().to_string(),
        )
    }
}

impl Neg for DataStateAttrValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            DataStateAttrValue::Active => DataStateAttrValue::Inactive,
            DataStateAttrValue::Inactive => DataStateAttrValue::Active,
        }
    }
}
