use dioxus::prelude::*;
use dioxus_components::{atoms::Separator, molecules::dropdown::*};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn DropdownPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..3 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<DropdownProps> {}
    )
}

#[component]
pub fn AbcComp(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx!(  )
}

impl DemoComponent for DropdownProps {
    fn title() -> &'static str {
        "Dropdown"
    }

    fn description() -> &'static str {
        ""
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let content1 = build_preview_component::<DropdownContentProps, _>(
            &state.read()[&2],
            DropdownContent,
            rsx!(
                p { "Content 1" }
                p { "Content 2" }
                Separator {}
                p { "Content 3" }
                p { "Content 4" }
            ),
        );

        let toggle = build_preview_component::<DropdownToggleProps, _>(
            &state.read()[&1],
            DropdownToggle,
            rsx!( "Dropdown" ),
        );

        let dropdown = build_preview_component::<DropdownProps, _>(
            &state.read()[&0],
            Dropdown,
            rsx!(
                { toggle },
                { content1 }
            ),
        );

        rsx!(
            { dropdown }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<DropdownProps> { index: 0, state, comp_props: DropdownProps::default() }
            CompPreviewSelector::<DropdownToggleProps> { index: 1, state, comp_props: DropdownToggleProps::default() }
            CompPreviewSelector::<DropdownContentProps> { index: 2, state, comp_props: DropdownContentProps::default() }
        )
    }
}
