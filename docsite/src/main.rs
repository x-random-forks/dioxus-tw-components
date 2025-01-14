#![allow(non_snake_case)]

use dioxus::prelude::*;
use ::manganis;

mod app;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    log::info!("{}", _STYLE);
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::WARN)
            .build(),
    );
    launch(crate::app::App);
}
