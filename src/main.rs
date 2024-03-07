#![allow(non_snake_case)]

use dioxus::prelude::*;

mod button;

use button::Button;

pub trait Component {
    fn view(self) -> Element;
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Trace).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );
    launch(App);
}

// fn anon(e: Event<MouseData>) {}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        button {
            class: "bg-green-500 text-white font-bold py-2 px-4 rounded",
            onclick: |_| { log::debug!("clicked") },
            "BUTTON"
        }
        Button {
            // check: true,
            onclick: |_e| {
                log::debug!("clicked");
            },
            "My Button"
        }
    }
}
