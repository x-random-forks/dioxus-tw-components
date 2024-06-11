use dioxus::prelude::*;
use dioxus_components::form::input::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn InputPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<InputProps> {}
    )
}

impl DemoComponent for InputProps {
    fn title() -> &'static str {
        "Input"
    }

    fn description() -> &'static str {
        "A customizable and interactive input component"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        let preview_comp =
            build_preview_component::<InputProps, _>(&state.read()[&0], Input, None);

        rsx!(
            { preview_comp }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<InputProps> { index: 0, state, comp_props: InputProps::default() }
        )
    }
}
