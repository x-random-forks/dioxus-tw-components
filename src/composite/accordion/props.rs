use crate::hooks::use_signal_unique_id;
use crate::types::*;
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::JsValue;

struct AccordionState(bool);

// TODO Check if this works with multiple accordions

#[props_component(id, class, children)]
pub fn Accordion(#[props(default = false)] multi_open: bool) -> Element {
    let class = tw_merge!(props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn AccordionItem() -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(AccordionState(false)));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn AccordionTrigger() -> Element {
    let button_closure = move |_: Event<MouseData>| {
        if read_accordion_state() {
            use_accordion_state().set(AccordionState(false));
        } else {
            use_accordion_state().set(AccordionState(true));
        }
    };

    let class = tw_merge!(props.base(), props.class);

    rsx!(
        button { class: class, id: props.id, onclick: button_closure, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn AccordionContent() -> Element {
    let mut elem_height = use_signal(|| "".to_string());
    let sig_id = use_signal_unique_id(props.id.clone());

    let final_height = match read_accordion_state() {
        true => elem_height(),
        false => "0".to_string(),
    };

    let onmounted = move |_| async move {
        match get_element_height(&sig_id()) {
            Ok(height) => {
                elem_height.set(format!("{}px", height));
            }
            Err(e) => {
                log::error!("Failed to get element height: {:?}", e);
            }
        }
    };

    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { id: props.id, class: class, height: final_height, onmounted: onmounted, {props.children} }
    )
}

fn get_element_height(sig_id: &str) -> Result<i32, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Failed to get document"))?;
    let element = document
        .get_element_by_id(sig_id)
        .ok_or_else(|| JsValue::from_str("Element not found"))?;

    Ok(element.scroll_height())
}

fn use_accordion_state() -> Signal<AccordionState> {
    use_context::<Signal<AccordionState>>()
}

fn read_accordion_state() -> bool {
    use_context::<Signal<AccordionState>>().read().0
}
