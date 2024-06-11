use dioxus::prelude::*;
use dioxus_components::molecules::lightswitch::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn LightSwitchPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<LightSwitchProps> {}
    )
}

impl DemoComponent for LightSwitchProps {
    fn title() -> &'static str {
        "LightSwitch"
    }

    fn description() -> &'static str {
        "Turn off the light !"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        let preview_comp =
            build_preview_component::<LightSwitchProps, _>(&state.read()[&0], LightSwitch, rsx!( "LightSwitch" ));

        rsx!(
            { preview_comp }
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<LightSwitchProps> { index: 0, state, comp_props: LightSwitchProps::default() }
        )
    }
}
