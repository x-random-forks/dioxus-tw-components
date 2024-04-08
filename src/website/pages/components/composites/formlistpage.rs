use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_components_bin::{
    atom::{button::*, input::*, label::*},
    composite::{formlist::*, select::*},
};

pub fn FormListPage() -> Element {
    let mut values = use_signal(HashMap::new);

    let mut group = Vec::<Element>::new();
    for i in 0..5 {
        group.push(rsx!(
            SelectGroup { name: "animal-{i}",
                SelectPlaceholder { "Select an animal" }
                SelectLabel { label: "Domestic",
                    SelectItem { value: "dog", "Dog" }
                    SelectItem { value: "cat", "Cat" }
                    SelectItem { value: "hamster", "Hamster" }
                    SelectItem { value: "none", disabled: true, "None" }
                }
                SelectLabel { label: "Wild", disabled: true,
                    SelectItem { value: "lion", "Lion" }
                    SelectItem { value: "tiger", "Tiger" }
                    SelectItem { value: "bear", "Bear" }
                }
            }
            SelectGroup { name: "number-animal-{i}",
                SelectPlaceholder { "Select number of animals" }
                SelectLabel { label: "1-10",
                    SelectItem { value: "1", "1" }
                    SelectItem { value: "2", "2" }
                    SelectItem { value: "3", "3" }
                    SelectItem { value: "4", "4" }
                    SelectItem { value: "5", "5" }
                    SelectItem { value: "6", "6" }
                    SelectItem { value: "7", "7" }
                    SelectItem { value: "8", "8" }
                    SelectItem { value: "9", "9" }
                    SelectItem { value: "10", "10" }
                }
            }
            Label { r#for: "name-animal-{i}", "Type animal name" }
            Input { r#type: "text", name: "name-animal-{i}", placeholder: "Animal name" }
        ));
    }

    rsx!(
        "FORM LIST PAGE"
            div {
                form {
                    id: "id-formlist",
                    onsubmit: move |event| {
                        log::debug!("Form values {:#?}", values());
                        log::debug!("Valid :{}", event.valid());
                        values.set(event.values());
                    },
                    FormList { group_vec: group }
                    Button { r#type: "submit", variant: ButtonVariant::Outline, "Submit" }
                }
            }
            div {
                "Form Values"
                div { class: "h-4", "{values:#?}" }
            }
    )
}
