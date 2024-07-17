pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;
pub mod theme;

use dioxus::prelude::*;
use dioxus_components::molecules::Toaster;
use theme::THEME_MANAGER;
use theme::ToStyle;

use crate::app::router::Route;

pub fn App() -> Element {
    rsx!(
        Toaster { 
            div {
                class: "relative bg-background text-foreground",
                style: THEME_MANAGER.read().to_style(),
                Router::<Route> {}
            }
        }
    )
}