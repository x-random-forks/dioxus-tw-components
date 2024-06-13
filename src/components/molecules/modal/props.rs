use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;
#[derive(Clone, Copy)]
struct ModalState {
    data_state_attr_value: DataStateAttrValue,
}

impl ModalState {
    fn new(is_open: bool) -> Self {
        Self {
            data_state_attr_value: if is_open {
                DataStateAttrValue::Active
            } else {
                DataStateAttrValue::Inactive
            },
        }
    }

    fn toggle(&mut self) {
        self.data_state_attr_value = -self.data_state_attr_value;
    }
}

impl IntoAttributeValue for ModalState {
    fn into_value(self) -> dioxus_core::AttributeValue {
        self.data_state_attr_value.into_value()
    }
}

/// Usage: \
/// ```ignore
/// Modal {
///     ModalTrigger {
///          "Open Modal"
///     }
///     ModalBackground {}
///     ModalContent {
///        div {
///             ModalClose { "X" }
///        }
///        div { class: "h4", "TITLE" }
///        div { class: "paragraph", "CONTENT" }
///    }
/// }
/// ```
#[props_component(children)]
pub fn Modal(#[props(default = false)] is_open: bool) -> Element {
    use_context_provider(|| Signal::new(ModalState::new(props.is_open)));

    rsx!(
        { props.children }
    )
}

#[props_component(class, id, children)]
pub fn ModalTrigger() -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let trigger_closure = move |_: Event<MouseData>| {
        state.write().toggle();
    };

    rsx!(
        div { class: props.class, id: props.id, onclick: trigger_closure, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalClose() -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let onclick = move |_: Event<MouseData>| {
        state.write().toggle();
    };

    rsx!(
        button { class: props.class, id: props.id, onclick,
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 256 256",
                width: "15",
                height: "15",
                class: "fill-foreground/60 hover:fill-foreground",
                path { d: "M202.82861,197.17188a3.99991,3.99991,0,1,1-5.65722,5.65624L128,133.65723,58.82861,202.82812a3.99991,3.99991,0,0,1-5.65722-5.65624L122.343,128,53.17139,58.82812a3.99991,3.99991,0,0,1,5.65722-5.65624L128,122.34277l69.17139-69.17089a3.99991,3.99991,0,0,1,5.65722,5.65624L133.657,128Z" }
            }
        }
    )
}

#[props_component(class, id, children)]
pub fn ModalContent(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default = Animation::Full)] animation: Animation,
) -> Element {
    let state = use_context::<Signal<ModalState>>();

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value(),
        None,
        false,
    ));

    rsx!(
        div { ..props.attributes, class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalBackground(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default = true)] interactive: bool,
    #[props(default)] color: Color,
    #[props(default = Animation::Full)] animation: Animation,
) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let modal_closure = move |_: Event<MouseData>| {
        if props.interactive {
            state.write().toggle();
        }
    };

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value(),
        None,
        false,
    ));

    rsx!(
        div {
            ..props.attributes,
            class: props.class,
            id: props.id,
            onclick: modal_closure,
            {props.children}
        }
    )
}
