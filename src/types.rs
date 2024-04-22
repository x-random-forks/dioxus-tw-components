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
}
