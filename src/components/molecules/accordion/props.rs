use crate::attributes::*;
use crate::hooks::use_element_scroll_height;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;

struct AccordionState {
    multi_open: bool,
    active_items: Vec<String>,
}

impl AccordionState {
    fn new(multi_open: bool) -> Self {
        Self {
            multi_open,
            active_items: Vec::with_capacity(1),
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

    fn is_active_to_attr_value(&self, id: String) -> AttributeValue {
        match self.active_items.contains(&id) {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct AccordionProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Control if multiple items can be open at the same time
    #[props(default = false)]
    multi_open: bool,

    children: Element,
}

/// The Accordion component divides the content into collapsible items \
/// Usage:
/// ```ignore
/// rsx!(Accordion {
///     AccordionItem {
///         AccordionTrigger { id: "acc-1", "Trigger 1" }
///         AccordionContent { id: "acc-1", "Content 1" }
///     }
///     AccordionItem {
///         AccordionTrigger { id: "acc-2", "Trigger 2" }
///         AccordionContent { id: "acc-2", "Content 2" }
///     }
/// })
/// ```
pub fn Accordion(mut props: AccordionProps) -> Element {
    use_context_provider(|| Signal::new(AccordionState::new(props.multi_open)));

    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct AccordionItemProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// Wrapper for the [AccordionTrigger] and [AccordionContent] components
pub fn AccordionItem(mut props: AccordionItemProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct AccordionTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    /// Determine if the accordion item is open by default
    #[props(optional, default = false)]
    is_open: bool,

    /// Decoration element that is displayed next to the trigger text, by default a chevron svg
    #[props(optional, default = default_trigger_decoration())]
    trigger_decoration: Element,

    children: Element,
}

/// The clickable element that toggles the visibility of the [AccordionContent] component
pub fn AccordionTrigger(mut props: AccordionTriggerProps) -> Element {
    props.update_class_attribute();

    let mut state = use_context::<Signal<AccordionState>>();

    let onmounted = move |_| async move {
        if props.is_open {
            state.write().add_id(props.id.read().clone());
        }
    };

    let button_closure = move |_: Event<MouseData>| {
        // If the current item is active, remove it from the list, effectively closing it
        if state.read().is_active(&*props.id.read()) {
            state.write().remove_id(props.id.read().clone());
        } else {
            // If the current item is not active
            // set it as the only active item if multi_open is false
            // or add it to the list of active items if multi_open is true
            if !state.read().multi_open {
                state.write().set_id(props.id.read().clone());
            } else {
                state.write().add_id(props.id.read().clone());
            }
        }
    };

    rsx!(
        button {
            ..props.attributes,
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            onclick: button_closure,
            onmounted: onmounted,
            div { {props.children} }
            {props.trigger_decoration}
        }
    )
}

fn default_trigger_decoration() -> Element {
    rsx!(
        svg {
            class: "fill-foreground/65 transition-transform transform duration-300 group-data-[state=active]:-rotate-180",
            width: 12,
            height: 12,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            path { d: "M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z" }
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct AccordionContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    #[props(default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

/// Collapsible element that is toggled by the [AccordionTrigger] component
pub fn AccordionContent(mut props: AccordionContentProps) -> Element {
    // This is the height of the element when visible, we need to calcul it before rendering it to have a smooth transition
    let mut elem_height = use_signal(|| "".to_string());

    props.update_class_attribute();

    let onmounted = move |_| async move {
        if props.animation == Animation::None {
            elem_height.set("auto".to_string());
            return;
        }

        match use_element_scroll_height(&props.id.read()) {
            Ok(height) => {
                elem_height.set(format!("{}px", height));
            }
            Err(e) => {
                log::error!("AccordionContent: Failed to get element height(id probably not set): setting it to auto: {:?}", e);
                elem_height.set("auto".to_string());
            }
        }
    };

    let state = use_context::<Signal<AccordionState>>();

    let final_height = match state.read().is_active(&props.id.read()) {
        true => elem_height(),
        false => "0".to_string(),
    };

    rsx!(
        div {
            ..props.attributes,
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            id: props.id,
            height: final_height,
            onmounted,
            {props.children}
        }
    )
}
