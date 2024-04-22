use crate::attributes::*;
use crate::hooks::{use_element_scroll_height, use_string_to_signal_string};
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

struct AccordionState {
    multi_open: bool,
    current_active: Vec<String>,
}

impl AccordionState {
    fn new(multi_open: bool) -> Self {
        Self {
            multi_open,
            current_active: Vec::new(),
        }
    }

    fn add_id(&mut self, id: String) {
        self.current_active.push(id);
    }

    fn remove_id(&mut self, id: String) {
        self.current_active.retain(|x| x != &id);
    }

    fn set_id(&mut self, id: String) {
        self.current_active.clear();
        self.current_active.push(id);
    }

    fn is_active(&self, id: &str) -> bool {
        self.current_active.contains(&id.to_string())
    }
}

/// The Accordion component divides the content into collapsible items
#[props_component(id, class, children)]
pub fn Accordion(
    /// Control if multiple items can be open at the same time
    #[props(default = false)]
    multi_open: bool,
) -> Element {
    let class = tw_merge!(props.class);

    use_context_provider(|| Signal::new(AccordionState::new(props.multi_open)));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

/// Wrapper for the [AccordionTrigger] and [AccordionContent] components
#[props_component(id, class, children)]
pub fn AccordionItem() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

/// The clickable element that toggles the visibility of the [AccordionContent] component
#[props_component(id, class, children)]
pub fn AccordionTrigger(
    /// Determine if the accordion item is open by default
    #[props(default = false)]
    is_open: bool,
    /// Decoration element that is displayed next to the trigger text, by default a chevron svg
    #[props(default = use_default_trigger_decoration())]
    trigger_decoration: Element,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut accordion_state = consume_context::<Signal<AccordionState>>();

    let sig_id = use_string_to_signal_string(props.id.clone());

    let onmounted = move |_| async move {
        if props.is_open {
            accordion_state.write().add_id(sig_id());
        }
    };

    let button_closure = move |_: Event<MouseData>| {
        // If the current item is active, remove it from the list, effectively closing it
        if accordion_state.read().is_active(&sig_id()) {
            accordion_state.write().remove_id(sig_id());
        } else {
            // If the current item is not active
            // set it as the only active item if multi_open is false
            // or add it to the list of active items if multi_open is true
            if !accordion_state.read().multi_open {
                accordion_state.write().set_id(sig_id());
            } else {
                accordion_state.write().add_id(sig_id());
            }
        }
    };

    let state = match accordion_state.read().is_active(&sig_id()) {
        true => "active",
        false => "inactive",
    };

    rsx!(
        button {
            "data-state": state,
            class: class,
            id: props.id,
            onclick: button_closure,
            onmounted: onmounted,
            {props.children},
            {props.trigger_decoration}
        }
    )
}

fn use_default_trigger_decoration() -> Element {
    rsx!(dioxus_free_icons::Icon {
        class: "transition-transform transform duration-300 group-data-[state=active]:-rotate-180",
        width: 24,
        height: 24,
        icon: dioxus_free_icons::icons::fi_icons::FiChevronUp
    })
}

/// Collapsible element that is toggled by the [AccordionTrigger] component
#[props_component(id, class, children)]
pub fn AccordionContent(#[props(default)] animation: Animation) -> Element {
    let class = tw_merge!(props.base(), props.animation(), props.class);

    // This is the height of the element when visible, we need to calcul it before rendering it to have a smooth transition
    let mut elem_height = use_signal(|| "".to_string());

    let sig_id = use_string_to_signal_string(props.id.clone());

    let onmounted = move |_| async move {
        if props.animation == Animation::None {
            elem_height.set("auto".to_string());
            return;
        }

        match use_element_scroll_height(&sig_id()) {
            Ok(height) => {
                elem_height.set(format!("{}px", height));
            }
            Err(e) => {
                log::error!("Failed to get element height: {:?}, setting it to auto", e);
                elem_height.set("auto".to_string());
            }
        }
    };

    let accordion_state = consume_context::<Signal<AccordionState>>();

    let (final_height, state) = match accordion_state.read().is_active(&sig_id()) {
        true => (elem_height(), "active"),
        false => ("0".to_string(), "inactive"),
    };

    rsx!(
        div {
            "data-state": state,
            id: props.id,
            class: class,
            height: final_height,
            onmounted: onmounted,
            {props.children}
        }
    )
}
