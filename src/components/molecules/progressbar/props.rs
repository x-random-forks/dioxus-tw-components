use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ProgressBarProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,

    children: Element,
}

pub fn ProgressBar(mut props: ProgressBarProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ProgressBarInnerProps {
    #[props(default = 50)]
    progress: u8,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,

    children: Element,
}

pub fn ProgressBarInner(mut props: ProgressBarInnerProps) -> Element {
    props.update_class_attribute();

    let progress = if props.progress > 100 {
        100
    } else {
        props.progress
    };

    rsx!(
        div { ..props.attributes, style: "width: {progress}%",
            div { {props.children} }
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct ProgressLabelProps {
    #[props(default = 50)]
    progress: u8,
    #[props(default = true)]
    show_percentage: bool,

    #[props(extends = span, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn ProgressLabel(mut props: ProgressLabelProps) -> Element {
    props.update_class_attribute();

    rsx!(
        span { ..props.attributes,
            "{props.progress.to_string()}"
            if props.show_percentage {
                "%"
            }
        }
    )
}
