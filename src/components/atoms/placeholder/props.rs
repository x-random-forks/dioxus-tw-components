use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct PlaceholderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
}

impl std::default::Default for PlaceholderProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
        }
    }
}

pub fn Placeholder(mut props: PlaceholderProps) -> Element {
    props.update_class_attribute();

    rsx!(div { ..props.attributes })
}
