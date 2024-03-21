#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod atom;
pub mod button;

pub trait Component {
    fn view(self) -> Element;
}
