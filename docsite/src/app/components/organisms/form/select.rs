use dioxus::prelude::*;
use dioxus_components::form::select::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn SelectPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<SelectGroupProps> {}
    )
}

impl DemoComponent for SelectGroupProps {
    fn title() -> &'static str {
        "Select"
    }

    fn description() -> &'static str {
        "Interactive dropdown list that allows users to select an option. This component provides a user-friendly way to choose from a list of options."
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let preview_comp = build_preview_component::<SelectGroupProps, _>(
            &state.read()[&0],
            SelectGroup,
            rsx!(
                SelectPlaceholder { "Placeholder" }
                SelectLabel { label: "Items" }
                SelectItem { "First item" }
                SelectItem { "Second Item" }
            ),
        );

        rsx!(
            { preview_comp }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<SelectGroupProps> { index: 0, state, comp_props: SelectGroupProps::default() }
        )
    }
}
