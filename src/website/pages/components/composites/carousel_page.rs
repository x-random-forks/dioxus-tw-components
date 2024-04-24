use dioxus::prelude::*;
use dioxus_components::components::composites::carousel::*;

pub fn CarouselPage() -> Element {
    rsx!(
        "CAROUSEL PAGE"
        Carousel { default_item_key: 0, is_circular: true,
            CarouselTrigger { next: false }
            CarouselWindow {
                CarouselContent { class: "", id: "carousel-1",
                    CarouselItem { item_key: 0,
                        div { class: "", "ITEM 1" }
                    }
                    CarouselItem { item_key: 1,
                        div { class: "", "ITEM 2" }
                    }
                    CarouselItem { item_key: 2,
                        div { class: "", "ITEM 3" }
                    }
                }
            }
            CarouselTrigger { next: true }
        }
    )
}
