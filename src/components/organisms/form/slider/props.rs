use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SliderProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    value: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,
    #[props(optional)]
    onmounted: EventHandler<Event<MountedData>>,

    #[props(default)]
    pub color: ReadOnlySignal<Color>,
}

pub fn Slider(mut props: SliderProps) -> Element {
    props.build_class();

    let oninput = move |event| props.oninput.call(event);

    let onmounted = move |event: Event<MountedData>| props.onmounted.call(event);

    rsx!(
        input {
            ..props.attributes,
            r#type: "range",
            value: props.value,
            onmounted,
            oninput
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SliderTicksProps {
    #[props(optional, default = 10)]
    step: i64,
    #[props(optional, default = 0)]
    min: i64,
    #[props(optional, default = 100)]
    max: i64,

    #[props(extends = datalist, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn SliderTicks(mut props: SliderTicksProps) -> Element {
    props.build_class();

    rsx!(
        datalist { ..props.attributes,
            for i in props.min..props.max {
                if i % props.step == 0 {
                    option { value: i }
                }
            }
            option { value: props.max }
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct SliderLabelProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default = 0)]
    value: i64,
    #[props(optional, default = 100)]
    max: i64,
}

pub fn SliderLabel(mut props: SliderLabelProps) -> Element {
    props.build_class();

    rsx!(
        div { ..props.attributes,
            {props.value.to_string()},
            " / "
            {props.max.to_string()}
        }
    )
}
