use dioxus::prelude::*;
use dioxus_components::form::slider::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn SliderPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<SliderProps> {}
    )
}

impl DemoComponent for SliderProps {
    fn title() -> &'static str {
        "Slider"
    }

    fn description() -> &'static str {
        "Allows users to select a value within a specified range"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        let preview_comp =
            build_preview_component::<SliderProps, _>(&state.read()[&0], Slider, None);

        rsx!(
            { preview_comp }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<SliderProps> { index: 0, state, comp_props: SliderProps::default() }
        )
    }
}
