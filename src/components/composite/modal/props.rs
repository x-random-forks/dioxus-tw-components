use dioxus::prelude::*;
use props_component_macro::props_component;
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
///        Button { "Open Modal" } // can be anything
///     }
///     ModalBackground {}
///     ModalContent {
///        div {
///             ModalClose { Button { "X" } } // can be anything
///        }
///        div { class: "h4", "TITLE" }
///        div { class: "paragraph", "CONTENT" }
///    }
/// }
/// ```
#[props_component(children)]
pub fn Modal(#[props(default = false)] is_open: bool) -> Element {
    use_context_provider(|| Signal::new(ModalState::new(props.is_open)));

    rsx!({ props.children })
}

#[props_component(class, id, children)]
pub fn ModalTrigger() -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut state = use_context::<Signal<ModalState>>();

    let trigger_closure = move |_: Event<MouseData>| {
        state.write().toggle();
    };

    rsx!(
        div { class: class, id: props.id, onclick: trigger_closure, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalClose() -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut state = use_context::<Signal<ModalState>>();

    let trigger_closure = move |_: Event<MouseData>| {
        state.write().toggle();
    };

    rsx!(
        div { class: class, id: props.id, onclick: trigger_closure, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalContent(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default = Animation::Full)] animation: Animation,
) -> Element {
    let class = tw_merge!(props.base(), props.animation(), props.class);

    let state = use_context::<Signal<ModalState>>();

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value(),
        None,
        false,
    ));

    rsx!(
        div { ..props.attributes, class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalBackground(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default = true)] interactive: bool,
    #[props(default = Animation::Full)] animation: Animation,
) -> Element {
    let class = tw_merge!(props.base(), props.animation(), props.class);

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
            class: "{class}",
            id: props.id,
            onclick: modal_closure,
            {props.children}
        }
    )
}
