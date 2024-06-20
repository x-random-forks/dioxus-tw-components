use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use crate::attributes::*;

struct TabsState(String);

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TabsProps {
    #[props(optional)] default_tab: ReadOnlySignal<String>,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element
}

pub fn Tabs(mut props: TabsProps) -> Element {
    use_context_provider(|| Signal::new(TabsState(props.default_tab.read().clone())));

    props.build_class();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TabsListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element
}

pub fn TabsList(mut props: TabsListProps) -> Element {
    props.build_class();
    
    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TabsTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    children: Element,
}

pub fn TabsTrigger(mut props: TabsTriggerProps) -> Element {
    let mut tab_state = use_context::<Signal<TabsState>>();

    props.build_class();

    let state = match tab_state.read().0 == *props.id.read() {
        true => "active",
        false => "inactive",
    };

    let onclick = move |_| {
        tab_state.set(TabsState(props.id.read().clone()));
    };

    rsx!(
        button { "data-state": state, ..props.attributes, onclick, { props.children } }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TabsContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    children: Element
}

pub fn TabsContent(mut props: TabsContentProps) -> Element {
    let tab_state = use_context::<Signal<TabsState>>();

    props.build_class();

    let (state, is_hidden) = match tab_state.read().0 == *props.id.read() {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx!(
        div { ..props.attributes, "data-state": state, hidden: is_hidden, { props.children } }
    )
}