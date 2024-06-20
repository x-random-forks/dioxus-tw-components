pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;

use dioxus::prelude::*;
use dioxus_components::molecules::Toaster;

use crate::app::router::Route;

pub fn App() -> Element {
    rsx!(
        Toaster { 
            div { class: "bg-background", Router::<Route> {} }
        }
    )
}
