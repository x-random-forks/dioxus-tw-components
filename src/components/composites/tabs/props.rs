use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

use crate::attributes::*;

struct TabsState(String);

#[props_component(class, id, children)]
pub fn Tabs(#[props(into)] default_tab: String) -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(TabsState(props.default_tab)));

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TabsList() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class, id: props.id, {props.children} }
    )
}

#[props_component(class, id, children)]
pub fn TabsTrigger() -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut tab_state = consume_context::<Signal<TabsState>>();

    let state = match tab_state.read().0 == props.id {
        true => "active",
        false => "inactive",
    };

    let onclick = move |_| {
        tab_state.set(TabsState(props.id.clone()));
    };

    rsx!(
        button { "data-state": state, class, onclick: onclick, { props.children } }
    )
}

#[props_component(class, id, children)]
pub fn TabsContent() -> Element {
    let tab_state = consume_context::<Signal<TabsState>>();

    let class = tw_merge!(props.base(), props.class);

    let (state, is_hidden) = match tab_state.read().0 == props.id {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx!(
        div {
            "data-state": state,
            class,
            hidden: is_hidden,
            id: props.id,
            { props.children }
        }
    )
}
