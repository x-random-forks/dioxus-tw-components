use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SpacerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

impl std::default::Default for SpacerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
        }
    }
}

pub fn Spacer(mut props: SpacerProps) -> Element {
    props.update_class_attribute();

    rsx!(div { ..props.attributes })
}
