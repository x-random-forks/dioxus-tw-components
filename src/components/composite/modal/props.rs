use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::types::*;

struct ModalState(bool);

#[props_component(children)]
pub fn Modal(#[props(default = true)] is_open: bool) -> Element {
    use_context_provider(|| Signal::new(ModalState(props.is_open)));

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
    let class = tw_merge!(props.base(), props.class);

    let modal_context = use_context::<Signal<ModalState>>();

    let (state, hidden) = match modal_context.read().0 {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx!(
        div { "data-state": state, class: class, id: props.id, hidden: hidden, {props.children} }
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

    let modal_context = use_context::<Signal<ModalState>>();

    let (state, hidden) = match modal_context.read().0 {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx!(
        div {
            "data-state": state,
            class: "{class}",
            id: props.id,
            hidden: hidden,
            onclick: modal_closure,
            {props.children}
        }
    )
}

fn toggle_modal(mut modal_context: Signal<ModalState>) {
    if modal_context.read().0 {
        modal_context.set(ModalState(false));
    } else {
        modal_context.set(ModalState(true));
    }
}
