use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::tw_merge;

#[derive(Clone, Copy)]
struct ToastState {
    active: bool,
    description: &'static str,
}

impl ToastState {
    fn new() -> Self {
        Self {
            active: false,
            description: "",
        }
    }

    fn set_description(&mut self, description: &'static str) {
        self.description = description;
    }
}

#[props_component(class, id, children)]
pub fn Toast() -> Element {
    let class = tw_merge!("fixed w-full");

    use_context_provider(|| Signal::new(ToastState::new()));

    rsx!(
        div { role: "toast", class, { props.children } }
    )
}

#[props_component(class, id, children)]
pub fn ToastTrigger(#[props(default)] description: &'static str) -> Element {
    let mut state = consume_context::<Signal<ToastState>>();

    let onclick = move |_| {
        log::debug!("ToastTrigger clicked");
    };

    rsx!( div {onclick, { props.children }})
}

#[props_component(class, id, children)]
pub fn ToastContent() -> Element {
    rsx!({ props.children })
}
