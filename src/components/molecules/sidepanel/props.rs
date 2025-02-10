use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;

#[derive(Clone, Copy)]
pub struct SidePanelState {
    is_active: bool,
}

impl SidePanelState {
    fn new(is_active: bool) -> Self {
        Self { is_active }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }
}

impl IntoAttributeValue for SidePanelState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelProps {
    #[props(default = false)]
    is_active: bool,

    children: Element,
}

impl std::default::Default for SidePanelProps {
    fn default() -> Self {
        Self {
            is_active: false,
            children: rsx! {},
        }
    }
}

pub fn SidePanel(props: SidePanelProps) -> Element {
    use_context_provider(|| Signal::new(SidePanelState::new(props.is_active)));

    rsx!({ props.children })
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for SidePanelTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

pub fn SidePanelTrigger(mut props: SidePanelTriggerProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
        props.onclick.call(event)
    };

    rsx!(
        div { onclick, ..props.attributes, {props.children} }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    children: Element,
}

impl std::default::Default for SidePanelCloseProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default()), // Default this way to be able to check the children in SidePanelClose
        }
    }
}

/// Div to close the content side panel, by default it is a cross svg located at the top left corner of the side panel
/// If you provide a children, it will be used instead of the default cross svg and no internal styling will be provided
pub fn SidePanelClose(mut props: SidePanelCloseProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    let has_children = props.children != Ok(VNode::default());

    if !has_children {
        props.update_class_attribute();
    }

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
    };

    rsx!(
        div { onclick, ..props.attributes,
            if !has_children {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 256 256",
                    class: "size-[32px] fill-foreground/60 hover:fill-foreground",
                    path { d: "M202.82861,197.17188a3.99991,3.99991,0,1,1-5.65722,5.65624L128,133.65723,58.82861,202.82812a3.99991,3.99991,0,0,1-5.65722-5.65624L122.343,128,53.17139,58.82812a3.99991,3.99991,0,0,1,5.65722-5.65624L128,122.34277l69.17139-69.17089a3.99991,3.99991,0,0,1,5.65722,5.65624L133.657,128Z" }
                }
            } else {
                {props.children}
            }
        }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    pub side: ReadOnlySignal<Side>,

    children: Element,
}

impl std::default::Default for SidePanelContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            side: ReadOnlySignal::<Side>::default(),
            children: rsx! {},
        }
    }
}

pub fn SidePanelContent(mut props: SidePanelContentProps) -> Element {
    let state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    rsx!(
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SidePanelBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for SidePanelBackgroundProps {
    fn default() -> Self {
        Self {
            interactive: true,
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

pub fn SidePanelBackground(mut props: SidePanelBackgroundProps) -> Element {
    let mut state = use_context::<Signal<SidePanelState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        if props.interactive {
            state.write().toggle();
            props.onclick.call(event)
        }
    };

    rsx!(
        div {
            "data-state": state.read().into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    )
}
