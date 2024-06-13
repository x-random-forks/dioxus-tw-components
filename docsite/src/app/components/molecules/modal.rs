use dioxus::prelude::*;
use dioxus_components::molecules::modal::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn ModalPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..4 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<ModalProps> {}
    )
}

impl DemoComponent for ModalProps {
    fn title() -> &'static str {
        "Modal"
    }

    fn description() -> &'static str {
        ""
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let trigger = build_preview_component::<ModalTriggerProps, _>(
            &state.read()[&0],
            ModalTrigger,
            rsx!( "Open" ),
        );

        let background = build_preview_component::<ModalBackgroundProps, _>(
            &state.read()[&1],
            ModalBackground,
            None,
        );

        let close =
            build_preview_component::<ModalCloseProps, _>(&state.read()[&2], ModalClose, rsx!( "X" ));

        let content = build_preview_component::<ModalContentProps, _>(
            &state.read()[&3],
            ModalContent,
            rsx!(
                div { { close } }
                div { class: "h4", "TITLE" }
                div { class: "paragraph", "CONTENT" }
            ),
        );

        rsx!(
            Modal { 
                {trigger},
                {background},
                {content}
            }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex flex-col",
                CompPreviewSelector::<ModalTriggerProps> { index: 0, state, comp_props: ModalTriggerProps::default() }
                CompPreviewSelector::<ModalBackgroundProps> { index: 1, state, comp_props: ModalBackgroundProps::default() }
                CompPreviewSelector::<ModalCloseProps> { index: 2, state, comp_props: ModalCloseProps::default() }
                CompPreviewSelector::<ModalContentProps> { index: 3, state, comp_props: ModalContentProps::default() }
            }
        )
    }
}
