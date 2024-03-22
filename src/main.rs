#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components_bin::atom::{
    button::button::{Button, ButtonSize, ButtonVariant},
    textarea::textarea::TextArea,
};

pub trait Component {
    fn view(self) -> Element;
}

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::WARN)
            .build(),
    );
    launch(App);
}

fn App() -> Element {
    // This should be a global context in something like an AppState
    // this is just for demonstration purpose
    let mut dark = use_signal(|| "".to_string());
    let lightswitch_closure = move |_| {
        log::debug!("LightSwitch clicked");
        if dark() == "" {
            dark.set("dark".to_string());
        } else {
            dark.set("".to_string());
        }
    };

    rsx!(
        body { class: "{dark} bg-background",
            div {
                Button { onclick: lightswitch_closure, "LightSwitch" }
            }
            TestButton {}
        }
    )
}

// With the override class implemented, the user could just do something like
// Button { class: Class("btn-primary btn-xl"), "My Button"}
fn TestButton() -> Element {
    rsx!(
        div { class: "",
            Button { "Default" }
            Button { variant: ButtonVariant::Primary, "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
        }
        div { class: "",
            Button { size: ButtonSize::Sm, "Sm" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Lg" }
            Button { size: ButtonSize::Xl, "Xl" }
        }
        div { class: "", TextArea {} }
    )
}
