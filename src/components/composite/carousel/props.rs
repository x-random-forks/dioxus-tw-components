use crate::{attributes::*, hooks::use_element_scroll_width};
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::JsValue;

struct CarouselState {
    carousel_size: u32,
    current_key: u32,
    content_size: i32,
    is_circular: bool,
    content_id: String,
    current_translation: i32,
}

impl CarouselState {
    fn new(default_item: u32, is_circular: bool) -> Self {
        Self {
            current_key: default_item,
            carousel_size: 0,
            content_size: 0,
            is_circular,
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
                self.content_size = width;
            }
            Err(e) => {
                self.content_size = 0;
                log::error!(
                    "Failed to get element width: {:?}, setting it to {}",
                    e,
                    self.content_size
                );
                log::error!(
                    "Is the id of the CarouselContent set and correct? {:?}",
                    self.content_id
                );
            }
        }
    }

    fn go_to_next_item(&mut self) {
        self.current_key += 1;
    }

    fn go_to_prev_item(&mut self) {
        self.current_key -= 1;
    }

    fn go_to_item(&mut self, key: u32) {
        self.current_key = key;
    }

    fn set_content_id(&mut self, id: String) {
        self.content_id = id;
    }

    fn is_current_key_eq_mine(&self, key: u32) -> StateAttribute {
        if self.current_key == key {
            StateAttribute::Active
        } else {
            StateAttribute::Inactive
        }
    }

    fn translate(&mut self) {
        self.set_current_translation(
            self.current_key as i32 * self.content_size / self.carousel_size.max(1) as i32,
        )
    }

    fn set_current_translation(&mut self, translation: i32) {
        self.current_translation = translation;
    }

    fn get_current_translation(&self) -> i32 {
        self.current_translation
    }
}

#[props_component(id, class, children)]
pub fn Carousel(
    #[props(default = 0)] default_item: u32,
    #[props(default = false)] is_circular: bool,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    use_context_provider(|| Signal::new(CarouselState::new(props.default_item, props.is_circular)));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn CarouselWindow() -> Element {
    let class = tw_merge!(props.base(), props.class);

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

/// You need to pass it an id for it to work
#[props_component(id, class, children)]
pub fn CarouselContent(#[props(default)] animation: Animation) -> Element {
    let class = tw_merge!(props.base(), props.animation(), props.class);

    let mut carousel_state = consume_context::<Signal<CarouselState>>();

    let sig_id = use_signal(|| props.id.clone());

    let onmounted = move |_| async move {
        // Useful when default item is not the first one
        carousel_state
            .write()
            .set_content_size(use_element_scroll_width(&sig_id()));
        carousel_state.write().translate();

        carousel_state.write().set_content_id(sig_id());
    };

    let style = use_memo(move || {
        format!(
            "transform: translateX(-{}px)",
            carousel_state.read().get_current_translation()
        )
    });

    rsx!(
        div { class: class, style: style, id: props.id, onmounted: onmounted, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn CarouselItem(
    /// Represent position in the carousel
    item_key: u32,
) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let mut carousel_state = consume_context::<Signal<CarouselState>>();

    let onmounted = move |_| {
        carousel_state.write().increment_carousel_size();
    };

    // REVIEW: Is this ok ? Should I extend this everywhere possible ?
    // Found a better way in dropdown to do it so just change it whereever I can
    let state = carousel_state.read().is_current_key_eq_mine(props.item_key);

    rsx!(
        div { "data-state": state, class: class, id: props.id, onmounted: onmounted, {props.children} }
    )
}

#[props_component(id, class)]
pub fn CarouselTrigger(#[props(default = false)] next: bool) -> Element {
    let class = tw_merge!(props.base(), props.class);

    let onclick = move |_| {
        let mut carousel_state = consume_context::<Signal<CarouselState>>();
        let content_id = carousel_state.read().content_id.clone();

        carousel_state
            .write()
            .set_content_size(use_element_scroll_width(&content_id));

        use_carousel(props.next, carousel_state);

        carousel_state.write().translate();
    };

    let icon = get_next_prev_icons(props.next);

    rsx!(
        button { class: class, id: props.id, onclick: onclick, {icon} }
    )
}

fn use_carousel(next: bool, mut carousel_state: Signal<CarouselState>) {
    let mut carousel_state = carousel_state.write();
    let current_key = carousel_state.current_key;
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
        true => rsx!(dioxus_free_icons::Icon {
            class: "",
            width: 24,
            height: 24,
            icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleRight
        }),
        false => rsx!(dioxus_free_icons::Icon {
            class: "",
            width: 24,
            height: 24,
            icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleLeft
        }),
    }
}
