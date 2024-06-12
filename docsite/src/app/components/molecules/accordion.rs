use dioxus::prelude::*;
use dioxus_components::molecules::accordion::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn AccordionPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..4 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<AccordionProps> {}
    )
}

impl DemoComponent for AccordionProps {
    fn title() -> &'static str {
        "Accordion"
    }

    fn description() -> &'static str {
        "Up and down !"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Accordion {
                class: &state.read()[&0].get_class(),
                override_class: &state.read()[&0].get_override_class(),
                AccordionItem { 
                    AccordionTrigger { id: "accordion-1", "Products" }
                    AccordionContent {
                        class: &state.read()[&1].get_class(),
                        override_class: &state.read()[&1].get_override_class(),
                        animation: state.read()[&1].get_animation(),
                        id: "accordion-1",
                        "Check out our latest products!"
                    }
                }
                AccordionItem { 
                    AccordionTrigger { id: "accordion-2", "Services" }
                    AccordionContent {
                        class: &state.read()[&2].get_class(),
                        override_class: &state.read()[&2].get_override_class(),
                        animation: state.read()[&2].get_animation(),
                        id: "accordion-2",
                        "Discover our range of services."
                    }
                }
                AccordionItem { 
                    AccordionTrigger { id: "accordion-3", "Testimonials" }
                    AccordionContent {
                        class: &state.read()[&3].get_class(),
                        override_class: &state.read()[&3].get_override_class(),
                        animation: state.read()[&3].get_animation(),
                        id: "accordion-3",
                        "Read what our customers have to say."
                    }
                }
            }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex flex-col",
                CompPreviewSelector::<AccordionProps> { index: 0, state, comp_props: AccordionProps::default() }
                CompPreviewSelector::<AccordionContentProps> { index: 1, state, comp_props: AccordionContentProps::default() }
                CompPreviewSelector::<AccordionContentProps> { index: 2, state, comp_props: AccordionContentProps::default() }
                CompPreviewSelector::<AccordionContentProps> { index: 3, state, comp_props: AccordionContentProps::default() }
            }
        )
    }
}
