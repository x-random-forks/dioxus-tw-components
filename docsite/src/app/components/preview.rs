use std::{collections::HashMap, str::FromStr};

use dioxus::prelude::*;
use dioxus_components::{
    attributes::*,
    form::{Input, SelectGroup, SelectItem, SelectPlaceholder},
};
use dioxus_core::DynamicNode;

use crate::app::doctrait::{DemoComponent, IntoVec};

#[component]
pub fn PreviewFull<T: DemoComponent + 'static>() -> Element {
    rsx!(
        h1 { class: "h1", {T::title()} }
        PreviewDemo::<T> {}
    )
}

#[component]
fn PreviewDemo<T: DemoComponent>() -> Element {
    rsx!(
        section { class: "w-full border border-orange-700 space-y-2",
            h2 { id: "preview-title", class: "sr-only", {T::title()} }
            div { id: "preview-header", class: "border border-yellow-500", {T::description()} }
            PreviewWindow { 
                PreviewWindowComponent { {T::BuildCompPreview()} }
                PreviewWindowSelectors { {T::BuildCompSelectors()} }
            }
            div { id: "preview-footer", class: "border", "Footer" }
        }
    )
}

pub fn build_preview_component<T, M>(
    state: &FieldPreview,
    render_fn: impl ComponentFunction<T, M>,
    comp_child: Element,
) -> Element
where
    T: Default + BuildClass + HasChildren + Properties,
    M: 'static,
{
    // let mut comp_props = T::default();

    // // comp_props.set_class(state.get_class());
    // // comp_props.set_override_class(state.get_override_class());

    // if comp_props.has_color() {
    //     comp_props.set_color(state.get_color());
    // }

    // if comp_props.has_size() {
    //     comp_props.set_size(state.get_size());
    // }

    // if comp_props.has_animation() {
    //     comp_props.set_animation(state.get_animation());
    // }

    // if comp_props.has_orientation() {
    //     comp_props.set_orientation(state.get_orientation());
    // }

    // if comp_props.has_children() {
    //     comp_props.set_children(comp_child);
    // }

    // let vcomp = comp_props.into_vcomponent(render_fn, "");
    // let dnode_comp = DynamicNode::Component(vcomp);

    // rsx!(
    //     { dnode_comp }
    // )

    rsx!(  )
}

#[component]
pub fn CompPreviewSelector<T: BuildClass + std::cmp::PartialEq + 'static>(
    index: i32,
    mut state: Signal<HashPreview>,
    comp_props: T,
    children: Element,
) -> Element {
    rsx!(
        div { class: "flex flex-row space-x-4",
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
            if comp_props.has_orientation() {
                Selector { state, index, selector_type: SelectorType::Orientation }
            }
        }
    )
}

#[component]
pub fn ClassSelector(state: Signal<HashPreview>, index: i32) -> Element {
    rsx!(
        div { id: "class-selector",
            Input {
                placeholder: "Tailwind class",
                value: state.read().get(&index).unwrap().get_class(),
                oninput: move |event: FormEvent| {
                    if let Some(field_preview) = state.write().get_mut(&index) {
                        field_preview.set_class(event.data().value());
                    }
                }
            }
        }
        div { id: "override-class-selector",
            Input {
                placeholder: "Override all tailwind class",
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
    Orientation,
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
            SelectorType::Orientation => Orientation::into_vec()
                .iter()
                .map(|s| s.to_string())
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
            SelectorType::Orientation => {
                field_preview.set_orientation(Orientation::from_str(value).unwrap_or_default());
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
                SelectorType::Orientation => {
                    "Orientation"
                }
            }
        )
    }
}

#[component]
pub fn Selector(
    state: Signal<HashPreview>,
    index: i32,
    selector_type: ReadOnlySignal<SelectorType>,
) -> Element {
    let id = format!("{}-selector", selector_type.to_string().to_lowercase());
    let options = selector_type.read().into_vec();

    rsx!(
        div { id, class: "min-w-24",
            SelectGroup {
                oninput: move |event: FormEvent| {
                    if let Some(field_preview) = state.write().get_mut(&index) {
                        selector_type.read().set_value(field_preview, &event.data().value());
                    }
                },
                SelectPlaceholder { {selector_type().to_string()} }
                for option in options {
                    SelectItem { value: option.to_lowercase(), {option} }
                }
            }
        }
    )
}

// This is to eventually swap to Radio instead of Select of Selector, Radio seems more messy tho
// for (_ , option) in options.iter().enumerate() {
//     Radio {
//         oninput: move |event: FormEvent| {
//             if let Some(field_preview) = state.write().get_mut(&index) {
//                 selector_type.read().set_value(field_preview, &event.data().value());
//             }
//         },
//         name: selector_type.to_string().to_lowercase(),
//         id: option.to_lowercase(),
//         value: option.to_lowercase(),
//         {option.clone()}
//     }
// }

#[component]
pub fn PreviewWindow(children: Element) -> Element {
    rsx!(
        div {
            id: "preview-window",
            class: "p-4 min-h-96 border border-border rounded-global-radius flex flex-col items-center space-y-8",
            { children }
        }
    )
}

#[component]
fn PreviewWindowComponent(children: Element) -> Element {
    rsx!(
        div {
            id: "preview-window-component",
            class: "min-h-64 min-w-80 grow flex items-center justify-center",
            {children}
        }
    )
}

#[component]
fn PreviewWindowSelectors(children: Element) -> Element {
    rsx!(
        div { id: "preview-window-selectors", class: "flex flex-col", {children} }
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

    pub fn class(mut self, class: String) -> Self {
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

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
}
