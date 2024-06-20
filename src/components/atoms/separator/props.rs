use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use crate::attributes::*;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SeparatorProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    
    #[props(optional, default)]
    pub orientation: ReadOnlySignal<Orientation>,
}

pub fn Separator(mut props: SeparatorProps
) -> Element {
    props.build_class();
    
    rsx!(
        div { ..props.attributes }
    )
}
