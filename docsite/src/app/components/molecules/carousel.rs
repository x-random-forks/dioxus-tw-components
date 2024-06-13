use dioxus::prelude::*;
use dioxus_components::atoms::button::Button;
use dioxus_components::molecules::carousel::*;
use docsite_macro::{extract_html};

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn CarouselPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("w-96".to_string()));
        hash.insert(
            1,
            FieldPreview::default().class("bg-primary/40".to_string()),
        );
        hash.insert(
            2,
            FieldPreview::default()
                .class("bg-gradient-to-r from-primary/40 to-secondary/40".to_string()),
        );
        hash.insert(
            3,
            FieldPreview::default().class("bg-secondary/40".to_string()),
        );
        Signal::new(hash)
    });

    rsx!(PreviewFull::<CarouselProps> {})
}

impl DemoComponent for CarouselProps {
    fn title() -> &'static str {
        "Carousel"
    }

    fn description() -> &'static str {
        ""
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Carousel {
                class: &state.read()[&0].get_class(),
                override_class: &state.read()[&0].get_override_class(),
                CarouselTrigger { next: false }
                CarouselWindow {
                    CarouselContent { id: "carousel-prev",
                        CarouselItem {
                            item_key: 0,
                            class: &state.read()[&1].get_class(),
                            override_class: &state.read()[&1].get_override_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 1"
                            }
                        }
                        CarouselItem {
                            item_key: 1,
                            class: &state.read()[&2].get_class(),
                            override_class: &state.read()[&2].get_override_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 2"
                            }
                        }
                        CarouselItem {
                            item_key: 2,
                            class: &state.read()[&3].get_class(),
                            override_class: &state.read()[&3].get_override_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 3"
                            }
                        }
                    }
                }
                CarouselTrigger { next: true }
            }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<CarouselProps> { index: 0, state, comp_props: CarouselProps::default() }
            CompPreviewSelector::<CarouselItemProps> { index: 1, state, comp_props: CarouselItemProps::default() }
            CompPreviewSelector::<CarouselItemProps> { index: 2, state, comp_props: CarouselItemProps::default() }
            CompPreviewSelector::<CarouselItemProps> { index: 3, state, comp_props: CarouselItemProps::default() }
        )
    }
}
