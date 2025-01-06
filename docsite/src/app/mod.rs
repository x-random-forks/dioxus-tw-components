pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;
pub mod theme;

use dioxus::prelude::*;
use dioxus_components::prelude::*;

use crate::app::router::Route;

pub fn App() -> Element {
    let theme_manager = use_context_provider(|| Signal::new(ThemeManager::default()));

    rsx!(
        Toaster {
            div {
                class: "relative bg-background text-foreground",
                style: theme_manager.read().to_style(),
                ThemePicker {}
                Router::<Route> {}
            }
        }
    )
}