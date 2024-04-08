use super::style::*;
use crate::Component;
use component_derive::Component;
use dioxus::prelude::*;
use tailwind_fuse::*;

struct ModalState(bool);

// Modal is used a wrapper for every other modal component, it also sets up a context for those components
#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalProps {
    children: Element,
}

impl Component for ModalProps {
    fn view(self) -> Element {
        use_context_provider(|| Signal::new(ModalState(false)));

        rsx!({ self.children })
    }
}

// Used to open the modal
#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalTriggerProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalTriggerProps {
    fn view(self) -> Element {
        let trigger_closure = move |_: Event<MouseData>| {
            toggle_modal(use_context::<Signal<ModalState>>());
        };

        let class = ModalTriggerClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", onclick: trigger_closure, { self.children } }
        )
    }
}

#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalCancelProps {
    children: Element,

    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalCancelProps {
    fn view(self) -> Element {
        let trigger_closure = move |_: Event<MouseData>| {
            toggle_modal(use_context::<Signal<ModalState>>());
        };

        let class = ModalCancelClass::builder().with_class(self.class);

        rsx!(
            div { class: "{class}", onclick: trigger_closure, {self.children} }
        )
    }
}

#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalContentProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalContentProps {
    fn view(self) -> Element {
        let class = ModalContentClass::builder().with_class(self.class);

        rsx!(
            div { class: "{modal_state_to_string()} {class}", {self.children} }
        )
    }
}

#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalBackgroundProps {
    // Set to true if you want the modal to close when the background is clicked
    #[props(default = true)]
    interactive: bool,
    // Usefull if you want to render an image or smth else as the background
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalBackgroundProps {
    fn view(self) -> Element {
        let modal_closure = move |_: Event<MouseData>| {
            if self.interactive {
                toggle_modal(use_context::<Signal<ModalState>>());
            }
        };

        let class = ModalBackgroundClass::builder().with_class(self.class);

        rsx!(
            div {
                class: "{modal_state_to_string()} {class}",
                onclick: modal_closure,
                {self.children}
            }
        )
    }
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
