use std::collections::HashMap;

use dioxus::prelude::*;

use dioxus_components::{atoms::Button, form::Input, molecules::tabs::*};

use crate::app::doctrait::DemoComp;

#[component]
pub fn TabsPage() -> Element {
    let factor_elem = 2;
    let nb_base_elem = 1;

    let mut elem = use_signal(|| Element::default());

    let mut current_elem = use_signal(|| nb_base_elem);

    let mut hash = use_signal(|| {
        let mut hash = HashMap::<i32, String>::new();
        for i in 0..nb_base_elem * factor_elem {
            hash.insert(i, String::new());
        }
        hash
    });

    use_memo(move || {
        if *current_elem.read() > nb_base_elem {
            let key_insert = (current_elem() - 1)* factor_elem;
            log::debug!("Inserting {}", key_insert);
            hash.write().insert(key_insert, String::new());
            hash.write().insert(key_insert + 1, String::new());
        }
    });

    use_effect(move || {
        elem.set(rsx!(
            Tabs { default_tab: "tab-0", id: "tab",
                TabsList { 
                    for i in 0..current_elem() {
                        TabsTrigger { key: "tt-{i}", id: format!("tab-{}", i), class: hash()[&{ i * 2 }].clone(),
                            "Tab "
                            {i.to_string()}
                        }
                    }
                }
                for i in 0..current_elem() {
                    TabsContent {key: "tc-{i}",
                        id: format!("tab-{}", i),
                        class: hash()[&((i * 2) + 1)].clone(),
                        "Content "
                        {i.to_string()}
                    }
                }
            }
            div { class: "flex flex-col w-64 mt-8 space-y-2",
                for i in 0..current_elem() {
                    div { class: "border",
                        p { class: "paragraph",
                            "class Trigger "
                            {i.to_string()}
                        }
                        Input {
                            class: "w-full",
                            r#type: "text",
                            value: hash()[&{ i * 2 }].clone(),
                            oninput: move |event: FormEvent| {
                                hash.write().insert(i * 2, event.data().value());
                            }
                        }
                        p { class: "paragraph",
                            "class Content "
                            {i.to_string()}
                        }
                        Input {
                            class: "w-full",
                            r#type: "text",
                            value: hash()[&(i * 2 + 1)].clone(),
                            oninput: move |event: FormEvent| {
                                hash.write().insert(i * 2 + 1, event.data().value());
                            }
                        }
                    }
                }
            }
        ))
    });

    rsx!(
        {elem},
        Button {
            class: "font-bold",
            onclick: move |_| {
                if current_elem() > 1 {
                    current_elem -= 1;
                }
            },
            "-"
        }
        Button { class: "font-bold", onclick: move |_| { current_elem += 1 }, "+" }
    )
}

impl DemoComp for TabsProps {
    fn title() -> &'static str {
        "Tabs"
    }

    fn preview_comp(demo_state: &crate::app::doctrait::DemoState) -> Element {
        rsx!(  )
    }

    fn select_attributes(demo_state: &crate::app::doctrait::DemoState) -> Element {
        rsx!(  )
    }
}
