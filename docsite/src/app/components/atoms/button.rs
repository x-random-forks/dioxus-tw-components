use dioxus::prelude::*;
use dioxus_components::atoms::{button::ButtonProps, Button};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ButtonPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<ButtonProps> {}
    )
}

impl DemoComponent for ButtonProps {
    fn title() -> &'static str {
        "Button"
    }

    fn description() -> &'static str {
        "A simple yet customizable button"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        let preview_comp =
            build_preview_component::<ButtonProps, _>(&state.read()[&0], Button, rsx!( "Button" ));

        rsx!(
            { preview_comp }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<ButtonProps> { index: 0, state, comp_props: ButtonProps::default() }
        )
    }
}