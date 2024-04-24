use std::sync::atomic::AtomicUsize;
use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue};

const ID_PREFIX: &str = "dx42-";

static UNIQUE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn use_unique_id() -> String {
    format!(
        "{}{}",
        ID_PREFIX,
        UNIQUE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}

pub fn use_window() -> Result<web_sys::Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))
}

pub fn use_document(window: &web_sys::Window) -> Result<web_sys::Document, JsValue> {
    window
        .document()
        .ok_or_else(|| JsValue::from_str("web_sys Failed to get document"))
}

pub fn use_element_by_id(id: &str) -> Result<web_sys::Element, JsValue> {
    let window = use_window()?;
    let document = use_document(&window)?;
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("web_sys Element not found"))
}

pub fn use_element_scroll_height(id: &str) -> Result<i32, JsValue> {
    let element = use_element_by_id(id)?;

    Ok(element.scroll_height())
}

pub fn use_element_scroll_width(id: &str) -> Result<i32, JsValue> {
    let element = use_element_by_id(id)?;

    Ok(element.scroll_width())
}

/// Wrapper around web_sys::window::set_timeout_with_callback_and_timeout_and_arguments_0()
/// timeout is in milliseconds
/// Returns the timeout id if successful
pub fn use_set_timeout(
    window: &web_sys::Window,
    callback: &Closure<dyn FnMut()>,
    timeout: i32,
) -> Result<i32, JsValue> {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .or_else(|_| Err(JsValue::from_str("Failed to set timeout")))
}

/// Wrapper around web_sys::window::clear_timeout_with_handle()
pub fn use_clear_timeout_id(window: &web_sys::Window, timeout_id: i32) {
    window.clear_timeout_with_handle(timeout_id);
}
