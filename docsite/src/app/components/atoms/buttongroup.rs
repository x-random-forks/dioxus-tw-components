use dioxus::prelude::*;
use dioxus_components::atoms::buttongroup::{ButtonGroup, ButtonGroupItem, ButtonGroupProps};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ButtonGroupPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<ButtonGroupProps> {}
    )
}

impl DemoComponent for ButtonGroupProps {
    fn comp_introduction() -> &'static str {
        "A simple group of buttons"
    }

    fn BuildCompPreview() -> Element {
        rsx!(
            ButtonGroup {
                ButtonGroupItem { "Button 1" }
                ButtonGroupItem { "Button 2" }
                ButtonGroupItem { "Button 3" }
            }
        )        
    }

    fn BuildCompSelectors() -> Element {
        rsx!()
    }
}
