use crate::{
    hooks::{use_element_scroll_width, use_string_to_signal_string},
    types::*,
};
use dioxus::prelude::*;
use props_component_macro::props_component;
use tailwind_fuse::*;

struct CarouselState {
    size: u32,
    current_key: u32,
    content_size: i32,
}

impl CarouselState {
    fn new(default_item: u32) -> Self {
        Self {
            current_key: default_item,
            size: 0,
            content_size: 0,
        }
    }

    fn set_content_size(&mut self, size: i32) {
        self.content_size = size;
    }
}

#[props_component(id, class, children)]
pub fn Carousel(#[props(default = 0)] default_item: u32) -> Element {
    let class = tw_merge!("", props.class);

    use_context_provider(|| Signal::new(CarouselState::new(props.default_item)));

    rsx!(
        div { class: class, id: props.id, {props.children} }
    )
}

#[props_component(id, class, children)]
pub fn CarouselContent() -> Element {
    let class = tw_merge!(
        "transform transition-transform duration-500 flex",
        props.class
    );

    let mut carousel_state = consume_context::<Signal<CarouselState>>();

    let sig_id = use_string_to_signal_string(props.id.clone());

    let onmounted = move |_| async move {
        match use_element_scroll_width(&sig_id()) {
            Ok(width) => {
                carousel_state.write().set_content_size(width);
            }
            Err(e) => {
                carousel_state.write().set_content_size(0);
                log::error!(
                    "Failed to get element width: {:?}, setting it to {}",
                    e,
                    carousel_state.read().content_size
                )
            }
        }
    };

    let style = use_memo(move || {
        let state = carousel_state.read();
        // Avoid division by zero
        let translation = state.current_key as i32 * state.content_size / state.size.max(1) as i32;
        format!("transform: translateX(-{}px)", translation)
    });

    rsx!(
        div { class: "relative overflow-hidden",
            div { class: class, style: style, id: props.id, onmounted: onmounted, {props.children} }
        }
    )
}

#[props_component(id, class, children)]
pub fn CarouselItem(
    /// Represents position in the carousel
    item_key: u32,
) -> Element {
    let class = tw_merge!("relative min-w-0 shrink-0 grow-0 basis-full", props.class);

    let mut carousel_state = consume_context::<Signal<CarouselState>>();

    let onmounted = move |_| {
        carousel_state.write().size += 1;
    };

    let state = if carousel_state.read().current_key == props.item_key {
        "active"
    } else {
        "inactive"
    };

    rsx!(
        div { "data-state": state, class: class, id: props.id, onmounted: onmounted, {props.children} }
    )
}

#[props_component(id, class)]
pub fn CarouselTrigger(#[props(default = false)] next: bool) -> Element {
    let class = tw_merge!("size-10", props.class);

    let onclick = move |_| {
        use_carousel(props.next, consume_context::<Signal<CarouselState>>());
    };

    let icon = match props.next {
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
    };

    rsx!(
        button { class: class, id: props.id, onclick: onclick, {icon} }
    )
}

fn use_carousel(next: bool, mut carousel_state: Signal<CarouselState>) {
    if next && carousel_state.read().current_key < carousel_state.read().size - 1 {
        carousel_state.write().current_key += 1;
    } else if !next && carousel_state.read().current_key > 0 {
        carousel_state.write().current_key -= 1;
    }
}
