pub mod components;
pub mod home;
pub mod layout;
pub mod router;
pub mod doctrait;

use dioxus::prelude::*;

use crate::app::router::Route;

pub fn App() -> Element {
    rsx!(
        div { class: "bg-background", Router::<Route> {} }
    )
}
