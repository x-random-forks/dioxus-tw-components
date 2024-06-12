use dioxus::prelude::*;
use dioxus_components::molecules::breadcrumb::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn BreadcrumbPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..6 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<BreadcrumbProps> {}
    )
}

impl DemoComponent for BreadcrumbProps {
    fn title() -> &'static str {
        "Breadcrumb"
    }

    fn description() -> &'static str {
        "You left some on your way here"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        
        let item_1 = build_preview_component::<BreadcrumbItemProps, _>(&state.read()[&1], BreadcrumbItem, rsx!( "Home" ));
        let sep_1 = build_preview_component::<BreadcrumbSeparatorProps, _>(&state.read()[&2], BreadcrumbSeparator, None);
        let item_2 = build_preview_component::<BreadcrumbItemProps, _>(&state.read()[&3], BreadcrumbItem, rsx!( "Library" ));
        let sep_2 = build_preview_component::<BreadcrumbSeparatorProps, _>(&state.read()[&4], BreadcrumbSeparator, None);
        let item_3 = build_preview_component::<BreadcrumbItemProps, _>(&state.read()[&5], BreadcrumbItem, rsx!( "Data" ));

        let full_comp =  build_preview_component::<BreadcrumbProps, _>(&state.read()[&0], Breadcrumb, rsx!(
            {item_1},
            {sep_1},
            {item_2},
            {sep_2},
            {item_3}
        ));

        rsx!(
            {full_comp}
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex flex-col",
                CompPreviewSelector::<BreadcrumbProps> { index: 0, state, comp_props: BreadcrumbProps::default() }
                CompPreviewSelector::<BreadcrumbItemProps> { index: 1, state, comp_props: BreadcrumbItemProps::default() }
                CompPreviewSelector::<BreadcrumbSeparatorProps> { index: 2, state, comp_props: BreadcrumbSeparatorProps::default() }
                CompPreviewSelector::<BreadcrumbItemProps> { index: 3, state, comp_props: BreadcrumbItemProps::default() }
                CompPreviewSelector::<BreadcrumbSeparatorProps> { index: 4, state, comp_props: BreadcrumbSeparatorProps::default() }
                CompPreviewSelector::<BreadcrumbItemProps> { index: 5, state, comp_props: BreadcrumbItemProps::default() }
            }
        )
    }
}
