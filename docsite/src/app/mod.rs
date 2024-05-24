pub mod components;
pub mod home;
pub mod layout;
pub mod router;

use dioxus::prelude::*;

use crate::app::router::Route;

pub fn App() -> Element {
    rsx!(
        Router::<Route> {}
    )
}
