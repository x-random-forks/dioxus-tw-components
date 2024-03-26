#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod atom;
pub mod styling;

pub trait Component {
    fn view(self) -> Element;
}
