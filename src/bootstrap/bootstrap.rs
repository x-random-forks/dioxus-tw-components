use dioxus::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub struct BootstrapConfig {
}

#[component]
pub fn DioxusTwComponentsBootstrap(config: BootstrapConfig) -> Element {
    rsx! {
        document::Stylesheet { href: "https://fonts.googleapis.com/icon?family=Material+Icons" }
    }
}
