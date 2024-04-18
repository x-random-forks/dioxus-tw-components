use dioxus::{hooks::use_signal, signals::Signal};
use std::sync::atomic::AtomicUsize;
use web_sys::wasm_bindgen::JsValue;

const ID_PREFIX: &str = "dx42-";

static UNIQUE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn use_unique_id() -> String {
    format!(
        "{}{}",
        ID_PREFIX,
        UNIQUE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}

/// Convert a String to a `Signal<String>`
pub fn use_string_to_signal_string(string: String) -> Signal<String> {
    use_signal(|| string)
}

pub fn use_element_scroll_height(sig_id: &str) -> Result<i32, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Failed to get document"))?;
    let element = document
        .get_element_by_id(sig_id)
        .ok_or_else(|| JsValue::from_str("Element not found"))?;

    Ok(element.scroll_height())
}
