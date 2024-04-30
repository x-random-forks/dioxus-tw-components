use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::tw_merge;
use web_sys::wasm_bindgen::closure::Closure;

use crate::hooks::{use_element_by_id, use_set_timeout, use_window};

pub struct ToastState {
    toast_count: u32,
}

impl ToastState {
    pub fn new() -> Self {
        Self { toast_count: 0 }
    }

    pub fn increment(&mut self) {
        self.toast_count += 1;
    }
}

#[props_component(class, children)]
pub fn Toast() -> Element {
    let class = tw_merge!("fixed top-0 right-0 m-4 z-50 w-48", props.class);

    use_context_provider(|| Signal::new(ToastState::new()));

    rsx!(
        ol { role: "alert", id: "dx-toast", class, {children} }
    )
}

#[props_component(class, id, children)]
pub fn ToastTrigger(
    #[props(default)] description: &'static str,
    #[props(default)] title: &'static str,
) -> Element {
    let mut state = consume_context::<Signal<ToastState>>();

    let onclick = move |_event: MouseEvent| {
        let element = use_element_by_id("dx-toast").unwrap();

        let toast_count = state.read().toast_count;
        let li_id = format!("dx-toast-li-{}", toast_count);
        let li_class =
            "bg-background text-foreground border rounded-global-radius p-small text-small";
        let html = format!(
            "<li id=\"{li_id}\" class=\"{li_class}\">{} {toast_count}</li>",
            props.description
        );

        element
            .insert_adjacent_html("afterbegin", html.as_str())
            .unwrap();

        state.write().increment();

        begin_toast_timeout(li_id);
    };

    rsx!(
        div { onclick, {children} }
    )
}

#[props_component(class, id, children)]
pub fn ToastContent() -> Element {
    rsx!(
        { props.children }
    )
}

fn begin_toast_timeout(id: String) {
    let window = match use_window() {
        Ok(window) => window,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };

    let element = use_element_by_id(id.as_str()).unwrap();

    let closure = Closure::wrap(Box::new(move || element.remove()) as Box<dyn FnMut()>);

    if let Ok(id) = use_set_timeout(&window, &closure, 5000) {
        log::debug!("Timeout ID: {:?}", id);
    }

    closure.forget();
}
