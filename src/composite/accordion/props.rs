use crate::{hooks::use_signal_unique_id, Component};
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::JsValue;

struct AccordionState(bool);

// TODO Add multi_open functionality
props!(AccordionProps {
    #[props(default = false)]
    multi_open: bool,
});

impl Component for AccordionProps {
    fn view(self) -> Element {
        rsx!({ self.children })
    }
}

props!(AccordionItemProps {});

impl Component for AccordionItemProps {
    fn view(self) -> Element {
        let class = super::style::AccordionItemClass::builder().with_class("");

        use_context_provider(|| Signal::new(AccordionState(false)));

        rsx!(
            div { class: "{class}", id: self.id, {self.children} }
        )
    }
}

props!(AccordionTriggerProps {});

impl Component for AccordionTriggerProps {
    fn view(self) -> Element {
        let button_closure = move |_: Event<MouseData>| {
            if read_accordion_state() {
                use_accordion_state().set(AccordionState(false));
            } else {
                use_accordion_state().set(AccordionState(true));
            }
        };

        let class = super::style::AccordionTriggerClass::builder().with_class(self.class);

        rsx!( button { class: "{class}", id: self.id, onclick: button_closure, "Button" } )
    }
}

props!(AccordionContentProps {});

impl Component for AccordionContentProps {
    fn view(self) -> Element {
        let mut elem_height = use_signal(|| "".to_string());
        let sig_id = use_signal_unique_id(self.id.clone());

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

        let class = super::style::AccordionContentClass::builder().with_class(self.class);

        rsx!(
            div {
                id: "{self.id}",
                class: "{class}",
                height: final_height,
                onmounted: onmounted,
                {self.children}
            }
        )
    }
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
