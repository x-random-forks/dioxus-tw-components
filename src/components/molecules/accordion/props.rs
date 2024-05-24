use crate::attributes::*;
use crate::hooks::use_element_scroll_height;
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

struct AccordionState {
    multi_open: bool,
    active_items: Vec<String>,
}

impl AccordionState {
    fn new(multi_open: bool) -> Self {
        Self {
            multi_open,
            active_items: Vec::new(),
        }
    }

    fn add_id(&mut self, id: String) {
        self.active_items.push(id);
    }

    fn remove_id(&mut self, id: String) {
        self.active_items.retain(|x| x != &id);
    }

    fn set_id(&mut self, id: String) {
        self.active_items.clear();
        self.active_items.push(id);
    }
    
    fn is_active(&self, id: &str) -> bool {
        self.active_items.contains(&id.to_string())
    }

    fn into_value_id(&self, id: &str) -> DataStateAttrValue {
        if self.active_items.contains(&id.to_string()) {
            DataStateAttrValue::Active
        } else {
            DataStateAttrValue::Inactive
        }
    }
}

/// The Accordion component divides the content into collapsible items \
/// Usage:
/// ```ignore
/// Accordion {
///     AccordionItem {
///         AccordionTrigger { "Trigger 1" }
///         AccordionContent { "Content 1" }
///     }
///     AccordionItem {
///         AccordionTrigger { "Trigger 2" }
///         AccordionContent { "Content 2" }
///     }
/// }
/// ```
#[props_component(id, class, children)]
pub fn Accordion(
    /// Control if multiple items can be open at the same time
    #[props(default = false)]
    multi_open: bool,
) -> Element {
    let class = tw_merge!(props.class);

    use_context_provider(|| Signal::new(AccordionState::new(props.multi_open)));

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

/// Wrapper for the [AccordionTrigger] and [AccordionContent] components
#[props_component(id, class, children)]
pub fn AccordionItem() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

/// The clickable element that toggles the visibility of the [AccordionContent] component
#[props_component(id, class, children)]
pub fn AccordionTrigger(
    #[props(extends = button)] mut attributes: Vec<Attribute>,
    /// Determine if the accordion item is open by default
    #[props(default = false)]
    is_open: bool,
    /// Decoration element that is displayed next to the trigger text, by default a chevron svg
    #[props(default = use_default_trigger_decoration())]
    trigger_decoration: Element,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut state = consume_context::<Signal<AccordionState>>();

    let sig_id = use_signal(|| props.id.clone());

    let onmounted = move |_| async move {
        if props.is_open {
            state.write().add_id(sig_id());
        }
    };

    let button_closure = move |_: Event<MouseData>| {
        // If the current item is active, remove it from the list, effectively closing it
        if state.read().is_active(&sig_id()) {
            state.write().remove_id(sig_id());
        } else {
            // If the current item is not active
            // set it as the only active item if multi_open is false
            // or add it to the list of active items if multi_open is true
            if !state.read().multi_open {
                state.write().set_id(sig_id());
            } else {
                state.write().add_id(sig_id());
            }
        }
    };

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value_id(&sig_id()),
        None,
        true,
    ));

    rsx!(
        button {
            ..props.attributes,
            class,
            id: props.id,
            onclick: button_closure,
            onmounted: onmounted,
            {props.children},
            {props.trigger_decoration}
        }
    )
}

fn use_default_trigger_decoration() -> Element {
    rsx!(
        svg {
            class: "transition-transform transform duration-300 group-data-[state=active]:-rotate-180",
            width: 16,
            height: 16,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            path { d: "M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z" }
        }
    )
}

/// Collapsible element that is toggled by the [AccordionTrigger] component
#[props_component(id, class, children)]
pub fn AccordionContent(
    #[props(extends = div)] mut attributes: Vec<Attribute>,
    #[props(default)] animation: Animation,
) -> Element {
    let class = tw_merge!(props.base(), props.animation(), props.class);

    // This is the height of the element when visible, we need to calcul it before rendering it to have a smooth transition
    let mut elem_height = use_signal(|| "".to_string());

    let sig_id = use_signal(|| props.id.clone());

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

    let state = consume_context::<Signal<AccordionState>>();

    let final_height = match state.read().is_active(&sig_id()) {
        true => elem_height(),
        false => "0".to_string(),
    };

    props.attributes.push(Attribute::new(
        "data-state",
        state.read().into_value_id(&sig_id()),
        None,
        true,
    ));

    rsx!(
        div {
            ..props.attributes,
            id: props.id,
            class,
            height: final_height,
            onmounted: onmounted,
            {props.children}
        }
    )
}
