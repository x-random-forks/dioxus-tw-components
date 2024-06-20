use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SpacerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn Spacer(mut props: SpacerProps) -> Element {
    props.build_class();

    rsx!(
        div { ..props.attributes }
    )
}
