use dioxus::prelude::*;
use myderive::dxcomp;
use tailwind_fuse::*;

use super::{TestBoxVariant, TestTextVariant};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    // TO DO : Success,
    Accent,
    Muted,
}

pub trait Colorable {
    fn color(&self) -> &'static str;
}

pub trait BaseClass {
    fn base(&self) -> &'static str;
}

pub trait Variation {
    fn variant(&self) -> &'static str;
}

////////////////////////////////////////////////////////////////////////////////////////

#[dxcomp(color)]
#[derive(Clone, Props, PartialEq)]
pub struct TestTextProps {
    // #[props(default = Color::Primary)]
    // pub(crate) color: Color,
    #[props(default)]
    pub(crate) variant: TestTextVariant,
}

pub fn TestText(props: TestTextProps) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.variant());

    log::debug!("class: {:?}", class);

    rsx!( div { class: class, "Hello World" } )
}

////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Props, PartialEq)]
pub struct TestBoxProps {
    #[props(default = Color::Destructive)]
    pub(crate) color: Color,
    #[props(default)]
    pub(crate) variant: TestBoxVariant,
}

pub fn TestBox(props: TestBoxProps) -> Element {
    let class = tw_merge!(props.base(), props.color(), props.variant());

    log::debug!("class: {:?}", class);

    rsx!(div { class: class })
}
