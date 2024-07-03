use crate::{attributes::*, hooks::use_element_scroll_width};
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use dioxus_core::AttributeValue;
use web_sys::wasm_bindgen::JsValue;

struct CarouselState {
    is_circular: bool,
    carousel_size: u32,
    // Use a key there so we can just +1 or -1 instead of having a vec
    current_item_key: u32,
    content_id: String,
    content_width: i32,
    current_translation: i32,
}

impl CarouselState {
    fn new(current_item_key: u32, is_circular: bool) -> Self {
        Self {
            current_item_key,
            is_circular,
            carousel_size: 0,
            content_width: 0,
            content_id: String::from(""),
            current_translation: 0,
        }
    }

    fn increment_carousel_size(&mut self) {
        self.carousel_size += 1;
    }

    fn set_content_size(&mut self, scroll_width: Result<i32, JsValue>) {
        match scroll_width {
            Ok(width) => {
                self.content_width = width;
            }
            Err(e) => {
                self.content_width = 0;
                log::error!(
                    "Failed to get element width: {:?}, setting it to {}",
                    e,
                    self.content_width
                );
                log::error!(
                    "Is the id of the CarouselContent set and correct? {:?}",
                    self.content_id
                );
            }
        }
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

    fn set_content_id(&mut self, id: String) {
        self.content_id = id;
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

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct CarouselProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 0)]
    default_item_key: u32,
    #[props(default = false)]
    is_circular: bool,

    children: Element,
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
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct CarouselWindowProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn CarouselWindow(mut props: CarouselWindowProps) -> Element {
    props.update_class_attribute();

    rsx!(
        div { ..props.attributes, {props.children} }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct CarouselContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    id: ReadOnlySignal<String>,

    #[props(default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

/// You need to pass it an id for it to work
pub fn CarouselContent(mut props: CarouselContentProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onmounted = move |_| async move {
        // Useful when default item is not the first one
        carousel_state
            .write()
            .set_content_size(use_element_scroll_width(&props.id.read()));

        carousel_state.write().translate();

        carousel_state
            .write()
            .set_content_id(props.id.read().clone());
    };

    let style = use_memo(move || {
        format!(
            "transform: translateX(-{}px)",
            carousel_state.read().get_current_translation()
        )
    });

    rsx!(
        div {
            ..props.attributes,
            style,
            id: props.id,
            onmounted,
            {props.children}
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct CarouselItemProps {
    /// Represent position in the carousel
    item_key: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

pub fn CarouselItem(mut props: CarouselItemProps) -> Element {
    let mut state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onmounted = move |_| {
        state.write().increment_carousel_size();
    };

    rsx!(
        div {
            ..props.attributes,
            "data-state": state.read().is_active_to_attr_value(props.item_key),
            onmounted,
            {props.children}
        }
    )
}

#[derive(Clone, Default, PartialEq, Props, UiComp)]
pub struct CarouselTriggerProps {
    #[props(default = false)]
    next: bool,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

pub fn CarouselTrigger(mut props: CarouselTriggerProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onclick = move |_| {
        let content_id = carousel_state.read().content_id.clone();

        carousel_state
            .write()
            .set_content_size(use_element_scroll_width(&content_id));

        scroll_carousel(props.next, carousel_state);

        carousel_state.write().translate();
    };

    let icon = get_next_prev_icons(props.next);

    rsx!(
        button { ..props.attributes, onclick, {icon} }
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
    } else {
        if current_key > 0 {
            carousel_state.go_to_prev_item();
        } else if carousel_state.is_circular {
            carousel_state.go_to_item(carousel_size - 1);
        }
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
