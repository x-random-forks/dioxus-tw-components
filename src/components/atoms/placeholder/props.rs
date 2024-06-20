use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use crate::attributes::*;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct PlaceholderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
}

pub fn Placeholder(mut props: PlaceholderProps) -> Element {
    props.build_class();

    rsx!(
        div { ..props.attributes }
    )
}
