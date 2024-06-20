use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct InputProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,

    #[props(default)]
    pub size: ReadOnlySignal<Size>,
    #[props(default)]
    pub color: ReadOnlySignal<Color>,
}

pub fn Input(mut props: InputProps) -> Element {
    props.build_class();
    
    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx! {
        input {
            ..props.attributes,
            value: props.value,
            onmounted,
            oninput
        }
    }
}
