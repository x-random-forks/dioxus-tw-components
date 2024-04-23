use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

#[derive(Default, Clone, PartialEq)]
pub enum TmpEnum {
    #[default]
    Inline,
    Vertical,
}

#[props_component(class, children)]
pub fn Tmp(#[props(default)] layout: TmpEnum) -> Element {
    let class = match props.layout {
        TmpEnum::Inline => "inline-flex",
        TmpEnum::Vertical => "block",
    };

    let class = tw_merge!(class, props.class);

    rsx!(
        div { class, { props.children } }
    )
}
