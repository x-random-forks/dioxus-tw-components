use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

struct ModalState(bool);

#[props_component(children)]
pub fn Modal() -> Element {
    use_context_provider(|| Signal::new(ModalState(false)));

    rsx!({ props.children })
}

#[props_component(class, id, children)]
pub fn ModalTrigger() -> Element {
    let trigger_closure = move |_: Event<MouseData>| {
        toggle_modal(use_context::<Signal<ModalState>>());
    };

    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, onclick: trigger_closure, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalClose() -> Element {
    let trigger_closure = move |_: Event<MouseData>| {
        toggle_modal(use_context::<Signal<ModalState>>());
    };

    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, onclick: trigger_closure, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalContent() -> Element {
    let class = tw_merge!(props.base(), props.class, modal_state_to_string());

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn ModalBackground(#[props(default = true)] interactive: bool) -> Element {
    let modal_closure = move |_: Event<MouseData>| {
        if props.interactive {
            toggle_modal(use_context::<Signal<ModalState>>());
        }
    };

    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: "{modal_state_to_string()} {class}", id: props.id, onclick: modal_closure, {props.children} }
    )
}

fn toggle_modal(mut modal_context: Signal<ModalState>) {
    if modal_context.read().0 {
        modal_context.set(ModalState(false));
    } else {
        modal_context.set(ModalState(true));
    }
}

fn modal_state_to_string() -> &'static str {
    match use_context::<Signal<ModalState>>().read().0 {
        true => "fixed",
        false => "hidden",
    }
}
