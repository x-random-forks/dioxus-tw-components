#![allow(non_snake_case)]

use dioxus::html::geometry::*;

pub mod attributes;

mod components;
pub use components::*;

pub mod hooks;

pub mod prelude;

pub struct LibState {
    last_click_coordinates: Coordinates,
}

impl LibState {
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
