use std::{collections::HashMap, str::FromStr};

use dioxus::prelude::*;
use dioxus_components::{
    attributes::*,
    form::{Input, SelectGroup, SelectItem},
};
use dioxus_core::DynamicNode;

use crate::app::doctrait::{DemoComponent, IntoVec};

#[component]
pub fn PreviewFull<T: DemoComponent + 'static>() -> Element {
    rsx!(
        h1 { class: "h1", {T::title()} }
        p { class: "paragraph", {T::description()} }
        PreviewDemo::<T> {}
    )
}

#[component]
fn PreviewDemo<T: DemoComponent>() -> Element {
    rsx!(
        section { class: "w-full border border-orange-700",
            h2 { class: "sr-only", {T::title()} }
            div { class: "border border-yellow-500", "header" }
            div { class: "w-full items-center justify-center", PreviewWindow { {T::build_comp_preview()} } }
            div { class: "border border-red-600", "footer" }
            PreviewSelectors { {T::build_comp_selectors()} }
        }
    )
}

pub fn build_preview_component<T, M>(
    state: &FieldPreview,
    render_fn: impl ComponentFunction<T, M>,
    comp_child: Element,
) -> Element
where
    T: Default + BuildClass + HasChildren + Properties + Named,
    M: 'static,
{
    let mut comp_props = T::default();

    comp_props.set_class(state.get_class());
    comp_props.set_override_class(state.get_override_class());

    if comp_props.has_color() {
        comp_props.set_color(state.get_color());
    }

    if comp_props.has_size() {
        comp_props.set_size(state.get_size());
    }

    if comp_props.has_animation() {
        comp_props.set_animation(state.get_animation());
    }

    if comp_props.has_children() {
        comp_props.set_children(comp_child);
    }

    let vcomp = comp_props.into_vcomponent(render_fn, T::name());
    let dnode_comp = DynamicNode::Component(vcomp);
    rsx!(
        { dnode_comp }
    )
}

#[component]
pub fn CompPreviewSelector<T: BuildClass + std::cmp::PartialEq + 'static>(
    index: i32,
    mut state: Signal<HashPreview>,
    comp_props: T,
    children: Element,
) -> Element {
    rsx!(
        ClassSelector { state, index }
        if comp_props.has_color() {
            Selector { state, index, selector_type: SelectorType::Color }
        }
        if comp_props.has_size() {
            Selector { state, index, selector_type: SelectorType::Size }
        }
        if comp_props.has_animation() {
            Selector { state, index, selector_type: SelectorType::Animation }
        }
    )
}

#[component]
pub fn ClassSelector(state: Signal<HashPreview>, index: i32) -> Element {
    rsx!(
        div { id: "class-selector", class: "",
            p { class: "paragraph", "Class" }
            Input {
                value: state.read().get(&index).unwrap().get_class(),
                oninput: move |event: FormEvent| {
                    if let Some(field_preview) = state.write().get_mut(&index) {
                        field_preview.set_class(event.data().value());
                    }
                }
            }
            p { class: "paragraph", "Override class" }
            Input {
                value: state.read().get(&index).unwrap().get_override_class(),
                oninput: move |event: FormEvent| {
                    if let Some(field_preview) = state.write().get_mut(&index) {
                        field_preview.set_override_class(event.data().value());
                    }
                }
            }
        }
    )
}

#[derive(Clone, PartialEq, Eq)]
pub enum SelectorType {
    Color,
    Size,
    Animation,
}

impl SelectorType {
    fn into_vec(&self) -> Vec<String> {
        match self {
            SelectorType::Color => Color::into_vec().iter().map(|c| c.to_string()).collect(),
            SelectorType::Size => Size::into_vec().iter().map(|s| s.to_string()).collect(),
            SelectorType::Animation => Animation::into_vec()
                .iter()
                .map(|a| a.to_string())
                .collect(),
        }
    }

    fn set_value(&self, field_preview: &mut FieldPreview, value: &str) {
        match self {
            SelectorType::Color => {
                field_preview.set_color(Color::from_str(value).unwrap_or_default());
            }
            SelectorType::Size => {
                field_preview.set_size(Size::from_str(value).unwrap_or_default());
            }
            SelectorType::Animation => {
                field_preview.set_animation(Animation::from_str(value).unwrap_or_default());
            }
        }
    }
}

impl std::fmt::Display for SelectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectorType::Color => {
                    "Color"
                }
                SelectorType::Size => {
                    "Size"
                }
                SelectorType::Animation => {
                    "Animation"
                }
            }
        )
    }
}

#[component]
pub fn Selector(state: Signal<HashPreview>, index: i32, selector_type: SelectorType) -> Element {
    let id = format!("{}-selector", selector_type.to_string().to_lowercase());
    let label = selector_type.to_string();
    let options = selector_type.into_vec();

    rsx!(
        div { id, class: "",
            p { class: "paragraph", {label.clone()} }
            SelectGroup {
                oninput: move |event: FormEvent| {
                    if let Some(field_preview) = state.write().get_mut(&index) {
                        selector_type.set_value(field_preview, &event.data().value());
                    }
                },
                for option in options {
                    SelectItem { value: option.to_lowercase(), {option} }
                }
            }
        }
    )
}

#[component]
pub fn PreviewWindow(children: Element) -> Element {
    rsx!(
        div { class: "p-4 min-h-64 border border-black flex justify-center items-center",
            { children }
        }
    )
}

#[component]
pub fn PreviewSelectors(children: Element) -> Element {
    rsx!(
        div { class: "grid grid-cols-4 gap- grid-auto-rows auto-flow-dense w-full",
            { children }
        }
    )
}

pub type HashPreview = HashMap<i32, FieldPreview>;

#[derive(Default, Clone)]
pub struct FieldPreview {
    class: String,
    override_class: String,
    color: Color,
    size: Size,
    animation: Animation,
    orientation: Orientation,
}

impl FieldPreview {
    pub fn get_class(&self) -> String {
        self.class.clone()
    }

    pub fn set_class(&mut self, class: String) {
        self.class = class;
    }

    pub fn class(&mut self, class:String) -> &Self {
        self.class = class;
        self
    }

    pub fn get_override_class(&self) -> String {
        self.override_class.clone()
    }

    pub fn set_override_class(&mut self, override_class: String) {
        self.override_class = override_class;
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_size(&self) -> Size {
        self.size.clone()
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn get_animation(&self) -> Animation {
        self.animation.clone()
    }

    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = animation;
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation.clone()
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
}
