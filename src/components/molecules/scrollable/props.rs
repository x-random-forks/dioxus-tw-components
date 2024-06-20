use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ScrollableProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub orientation: ReadOnlySignal<Orientation>,

    children: Element,
}

pub fn Scrollable(mut props: ScrollableProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}
