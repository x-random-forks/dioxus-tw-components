use dioxus::prelude::*;
use dioxus_components_bin::components::composite::carousel::*;

pub fn CarouselPage() -> Element {
    rsx!(
        "CAROUSEL PAGE"
        Carousel { default_item: 0,
            CarouselTrigger { next: false }
            CarouselContent { class: "",
                CarouselItem { item_key: 0, div { class: "", "ITEM 1" } }
                CarouselItem { item_key: 1, div { class: "", "ITEM 2" } }
                CarouselItem { item_key: 2, div { class: "", "ITEM 3" } }
            }
            CarouselTrigger { next: true }
        }
    )
}
