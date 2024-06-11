use dioxus::prelude::*;
use props_component_macro::{props_component, BuildClass};
use tailwind_fuse::*;

use crate::attributes::*;

struct TabsState(String);

#[props_component(class, id, children)]
pub fn Tabs(#[props(optional, default = "tabs-0".to_string())] default_tab: String) -> Element {
    use_context_provider(|| Signal::new(TabsState(props.default_tab)));

    rsx!(
        div { class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TabsList() -> Element {
    rsx!(
        div { class: props.class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TabsTrigger() -> Element {
    let mut tab_state = use_context::<Signal<TabsState>>();

    let state = match tab_state.read().0 == props.id {
        true => "active",
        false => "inactive",
    };

    let onclick = move |_| {
        tab_state.set(TabsState(props.id.clone()));
    };

    rsx!(
        button { "data-state": state, class: props.class, onclick: onclick, { props.children } }
    )
}

#[props_component(class, id, children)]
pub fn TabsContent() -> Element {
    let tab_state = use_context::<Signal<TabsState>>();

    let (state, is_hidden) = match tab_state.read().0 == props.id {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx!(
        div {
            "data-state": state,
            class: props.class,
            hidden: is_hidden,
            id: props.id,
            { props.children }
        }
    )
}

impl Named for TabsProps {
    const NAME: &'static str = "Tabs";
}

impl Named for TabsTriggerProps {
    const NAME: &'static str = "TabsTrigger";
}

impl Named for TabsContentProps {
    const NAME: &'static str = "TabsContent";
}