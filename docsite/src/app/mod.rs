pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;

use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::router::Route;

pub fn App() -> Element {
    let theme_manager = use_context_provider(|| Signal::new(ThemeManager::default()));

    rsx!(
        document::Stylesheet {
            href: asset!(
                "/public/tailwind.css", CssAssetOptions::new().with_preload(true)
                .with_minify(false)
            ),
        }
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
