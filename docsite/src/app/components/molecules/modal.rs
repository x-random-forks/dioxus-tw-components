use dioxus::prelude::*;
use dioxus_components::molecules::modal::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn ModalPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..6 {
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
        
        rsx!(  )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex flex-col" }
        )
    }
}
