use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct SeparatorProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub orientation: ReadOnlySignal<Orientation>,
}

pub fn Separator(mut props: SeparatorProps) -> Element {
    props.update_class_attribute();

    rsx!(div { ..props.attributes })
}
