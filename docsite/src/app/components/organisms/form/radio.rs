use dioxus::prelude::*;
use dioxus_components::form::radio::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn RadioPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<RadioProps> {}
    )
}

impl DemoComponent for RadioProps {
    fn title() -> &'static str {
        "Radio"
    }

    fn description() -> &'static str {
        "A customizable and interactive radio component"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        
        let mut vec_prev_comp = Vec::<Element>::new();
        for index in 0..3 {
            vec_prev_comp.push(build_preview_component::<RadioProps, _>(
                &state.read()[&index],
                Radio,
                None,
            ));
        }

        rsx!(
            div { class: "flex space-x-2",
                for (index , prev_comp) in vec_prev_comp.iter().enumerate() {
                    div { class: "flex flex-col items-center space-y-2",
                        p { class: "text-foreground font-bold", {index.to_string()} }
                        {
                            prev_comp
                        }
                    }
                }
            }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex flex-col w-full",
                for index in 0..3 {
                    div { class: "flex flex-row space-x-2",
                        p { class: "text-foreground font-bold", {index.to_string()} }
                        CompPreviewSelector::<RadioProps> { index, state, comp_props: RadioProps::default() }
                    }
                }
            }
        )
    }
}
