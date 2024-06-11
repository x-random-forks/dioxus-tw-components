use dioxus::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};
use dioxus_components::molecules::tabs::*;

const NB_TABS: u8 = 3;

#[component]
pub fn TabsPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("w-96".to_string()).clone());
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<TabsProps> {}
    )
}

impl DemoComponent for TabsProps {
    fn title() -> &'static str {
        "Tabs"
    }

    fn description() -> &'static str {
        "A customizable and user-friendly navigation component that allows users to switch between different views or sections of a website or application"
    }

    fn build_comp_preview() -> Element {
        let state = use_context::<Signal<HashPreview>>();


        let preview_comp = build_preview_component::<TabsProps, _>(&state.read()[&0], Tabs, rsx!(
            TabsList { 
                for i in 0..NB_TABS {
                    TabsTrigger { id: format!("tabs-{i}"),
                        "Tab "
                        {i.to_string()}
                    }
                }
            }
            for i in 0..NB_TABS {
                TabsContent { id: format!("tabs-{i}"),
                    "Content "
                    {i.to_string()}
                }
            }
        ));

        rsx!(
            {preview_comp}
        )
    }

    fn build_comp_selectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<TabsProps> { index: 0, state, comp_props: TabsProps::default() }
        )
    }
}

// let factor_elem = 2;
// let nb_base_elem = 1;

// let mut elem = use_signal(|| Element::default());

// let mut current_elem = use_signal(|| nb_base_elem);

// let mut hash = use_signal(|| {
//     let mut hash = HashMap::<i32, String>::new();
//     for i in 0..nb_base_elem * factor_elem {
//         hash.insert(i, String::new());
//     }
//     hash
// });

// // !!
// let ab: TabsListProps = TabsListProps::default();
// let a = ab.color();
// if a.is_none() {
// }

// use_memo(move || {
//     if *current_elem.read() > nb_base_elem {
//         let key_insert = (current_elem() - 1)* factor_elem;
//         hash.write().insert(key_insert, String::new());
//         hash.write().insert(key_insert + 1, String::new());
//     }
// });

// use_effect(move || {
//     elem.set(rsx!(
//         Tabs { default_tab: "tab-0", id: "tab",
//             TabsList {
//                 for i in 0..current_elem() {
//                     TabsTrigger { key: "tt-{i}", id: format!("tab-{}", i), class: hash()[&{ i * 2 }].clone(),
//                         "Tab "
//                         {i.to_string()}
//                     }
//                 }
//             }
//             for i in 0..current_elem() {
//                 TabsContent {key: "tc-{i}",
//                     id: format!("tab-{}", i),
//                     class: hash()[&((i * 2) + 1)].clone(),
//                     "Content "
//                     {i.to_string()}
//                 }
//             }
//         }
//         div { class: "flex flex-col w-64 mt-8 space-y-2",
//             for i in 0..current_elem() {
//                 div { class: "border",
//                     p { class: "paragraph",
//                         "class Trigger "
//                         {i.to_string()}
//                     }
//                     Input {
//                         class: "w-full",
//                         r#type: "text",
//                         value: hash()[&{ i * 2 }].clone(),
//                         oninput: move |event: FormEvent| {
//                             hash.write().insert(i * 2, event.data().value());
//                         }
//                     }
//                     p { class: "paragraph",
//                         "class Content "
//                         {i.to_string()}
//                     }
//                     Input {
//                         class: "w-full",
//                         r#type: "text",
//                         value: hash()[&(i * 2 + 1)].clone(),
//                         oninput: move |event: FormEvent| {
//                             hash.write().insert(i * 2 + 1, event.data().value());
//                         }
//                     }
//                 }
//             }
//         }
//     ))
// });

// rsx!(
//     {elem},
//     Button {
//         class: "font-bold",
//         onclick: move |_| {
//             if current_elem() > 1 {
//                 current_elem -= 1;
//             }
//         },
//         "-"
//     }
//     Button { class: "font-bold", onclick: move |_| { current_elem += 1 }, "+" }
// )
