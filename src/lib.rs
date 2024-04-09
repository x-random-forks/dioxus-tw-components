#![allow(non_snake_case)]

use dioxus::prelude::*;

#[macro_use]
mod props;

#[macro_use]
mod class;

pub mod atom;
pub mod composite;
pub mod hooks;
pub mod layout;

pub trait Component {
    fn view(self) -> Element;
}
