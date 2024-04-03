use dioxus::prelude::*;
use dioxus_components_bin::composite::lightswitch::LightSwitchSignal;

use crate::website::router::Route;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(LightSwitchSignal("".to_string())));

    let light_switch_context = use_context::<Signal<LightSwitchSignal>>();
    let dark = &light_switch_context.read().0;
    rsx!(
        div { class: "{dark} bg-background text-foreground min-h-screen flex flex-col",
            Router::<Route> {}
        }
    )
}

#[component]
pub fn HomePage() -> Element {
    rsx!(
        section { class: "mx-auto flex flex-col items-center", div { "Hello, World!" } }
    )
}
