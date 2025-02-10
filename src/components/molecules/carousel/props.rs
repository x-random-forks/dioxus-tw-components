use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;
use std::rc::Rc;

use crate::attributes::*;

struct CarouselState {
    is_circular: bool,
    carousel_size: u32,
    // Use a key there so we can just +1 or -1 instead of having a vec
    current_item_key: u32,
    content_width: i32,
    current_translation: i32,
    content_data: Option<Rc<MountedData>>,
}

impl CarouselState {
    fn new(current_item_key: u32, is_circular: bool) -> Self {
        Self {
            current_item_key,
            is_circular,
            carousel_size: 0,
            content_width: 0,
            content_data: None,
            current_translation: 0,
        }
    }

    fn increment_carousel_size(&mut self) {
        self.carousel_size += 1;
    }

    fn set_content_size(&mut self, scroll_width: i32) {
        self.content_width = scroll_width;
    }

    fn go_to_next_item(&mut self) {
        self.current_item_key += 1;
    }

    fn go_to_prev_item(&mut self) {
        self.current_item_key -= 1;
    }

    fn go_to_item(&mut self, item_key: u32) {
        self.current_item_key = item_key;
    }

    fn set_content_element(&mut self, data: Option<Rc<MountedData>>) {
        self.content_data = data;
    }

    fn is_active_to_attr_value(&self, key: u32) -> AttributeValue {
        match self.current_item_key == key {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }

    fn translate(&mut self) {
        self.set_current_translation(
            self.current_item_key as i32 * self.content_width / self.carousel_size.max(1) as i32,
        )
    }

    fn set_current_translation(&mut self, translation: i32) {
        self.current_translation = translation;
    }

    fn get_current_translation(&self) -> i32 {
        self.current_translation
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 0)]
    default_item_key: u32,
    #[props(default = false)]
    is_circular: bool,

    children: Element,
}

impl std::default::Default for CarouselProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            default_item_key: 0,
            is_circular: false,
            children: rsx! {},
        }
    }
}

/// Usage :
/// ```ignore
/// Carousel { default_item_key: 0,
///     CarouselTrigger { next: false }
///     CarouselWindow {
///         CarouselContent { id: "carousel-1",
///             CarouselItem { item_key: 0, div { "ITEM 1" } }
///             CarouselItem { item_key: 1, div { "ITEM 2" } }
///         }
///     }
///     CarouselTrigger { next: true }
/// }
pub fn Carousel(mut props: CarouselProps) -> Element {
    use_context_provider(|| {
        Signal::new(CarouselState::new(
            props.default_item_key,
            props.is_circular,
        ))
    });

    props.update_class_attribute();

    rsx!(
        div { ..props.attributes,{props.children} }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselWindowProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for CarouselWindowProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn CarouselWindow(mut props: CarouselWindowProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes,{props.children} }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    id: ReadOnlySignal<String>,

    #[props(default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for CarouselContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

/// You need to pass it an id for it to work
pub fn CarouselContent(mut props: CarouselContentProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let style = use_memo(move || {
        format!(
            "transform: translateX(-{}px)",
            carousel_state.read().get_current_translation()
        )
    });

    rsx!(
        div {
            style,
            id: props.id,
            onmounted: move |element| async move {
                carousel_state.write().set_content_element(Some(element.data()));
                carousel_state
                    .write()
                    .set_content_size(match element.data().get_scroll_size().await {
                        Ok(size) => size.width as i32,
                        Err(_) => 0
                    });

                carousel_state.write().translate();
            },
            ..props.attributes,
            {props.children}
        }
    )
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselItemProps {
    /// Represent position in the carousel
    item_key: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for CarouselItemProps {
    fn default() -> Self {
        Self {
            item_key: 0,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn CarouselItem(mut props: CarouselItemProps) -> Element {
    let mut state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onmounted = move |_| {
        state.write().increment_carousel_size();
    };

    rsx!(
        div {
            "data-state": state.read().is_active_to_attr_value(props.item_key),
            onmounted,
            ..props.attributes,
            {props.children}
        }
    )
}

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct CarouselTriggerProps {
    #[props(default = false)]
    next: bool,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn CarouselTrigger(mut props: CarouselTriggerProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onclick = move |_| async move {
        let content_data = carousel_state.read().content_data.clone();

        if let Some(data) = content_data {
            carousel_state
                .write()
                .set_content_size(match data.get_scroll_size().await {
                    Ok(size) => size.width as i32,
                    Err(_) => 0,
                });
        }

        scroll_carousel(props.next, carousel_state);

        carousel_state.write().translate();
    };

    let icon = get_next_prev_icons(props.next);

    rsx!(
        button { onclick, ..props.attributes, {icon} }
    )
}

fn scroll_carousel(next: bool, mut carousel_state: Signal<CarouselState>) {
    let mut carousel_state = carousel_state.write();
    let current_key = carousel_state.current_item_key;
    let carousel_size = carousel_state.carousel_size;

    if next {
        if current_key < carousel_size - 1 {
            carousel_state.go_to_next_item();
        } else if carousel_state.is_circular {
            carousel_state.go_to_item(0);
        }
    } else if current_key > 0 {
        carousel_state.go_to_prev_item();
    } else if carousel_state.is_circular {
        carousel_state.go_to_item(carousel_size - 1);
    }
}

fn get_next_prev_icons(is_next: bool) -> Element {
    match is_next {
        true => rsx!(
            svg {
                width: 24,
                height: 24,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 512 512",
                class: "fill-foreground",
                path { d: "M294.6 135.1c-4.2-4.5-10.1-7.1-16.3-7.1C266 128 256 138 256 150.3V208H160c-17.7 0-32 14.3-32 32v32c0 17.7 14.3 32 32 32h96v57.7c0 12.3 10 22.3 22.3 22.3c6.2 0 12.1-2.6 16.3-7.1l99.9-107.1c3.5-3.8 5.5-8.7 5.5-13.8s-2-10.1-5.5-13.8L294.6 135.1z" }
            }
        ),
        false => rsx!(
            svg {
                width: 24,
                height: 24,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 512 512",
                class: "fill-foreground",
                transform: "scale(-1, 1)",
                path { d: "M294.6 135.1c-4.2-4.5-10.1-7.1-16.3-7.1C266 128 256 138 256 150.3V208H160c-17.7 0-32 14.3-32 32v32c0 17.7 14.3 32 32 32h96v57.7c0 12.3 10 22.3 22.3 22.3c6.2 0 12.1-2.6 16.3-7.1l99.9-107.1c3.5-3.8 5.5-8.7 5.5-13.8s-2-10.1-5.5-13.8L294.6 135.1z" }
            }
        ),
    }
}
