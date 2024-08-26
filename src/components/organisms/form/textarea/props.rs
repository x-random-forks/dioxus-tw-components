use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct TextAreaProps {
    #[props(extends = textarea, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
}

pub fn TextArea(mut props: TextAreaProps) -> Element {
    props.update_class_attribute();

    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx!(
        textarea {
            ..props.attributes.clone(),
            value: props.value,
            onmounted,
            oninput
        }
    )
}
