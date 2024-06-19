use dioxus::prelude::*;
use dioxus_components::form::textarea::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn TextAreaPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<TextAreaProps> {}
    )
}

impl DemoComponent for TextAreaProps {
    fn title() -> &'static str {
        "Textarea"
    }

    fn description() -> &'static str {
        "A customizable and interactive textarea component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        let preview_comp =
            build_preview_component::<TextAreaProps, _>(&state.read()[&0], TextArea, None);

        rsx!(
            { preview_comp }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<TextAreaProps> { index: 0, state, comp_props: TextAreaProps::default() }
        )
    }
}
