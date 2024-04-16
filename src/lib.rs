#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
pub mod hooks;
pub mod layout;
pub mod types;

pub trait Component {
    fn view(self) -> Element;
}
