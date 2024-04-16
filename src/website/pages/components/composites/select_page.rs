use dioxus::prelude::*;
use dioxus_components_bin::composite::select::*;

pub fn SelectPage() -> Element {
    rsx!(
        "SELECT PAGE"
        div { class: "flex w-96 gap-4 text-base",
            SelectGroup { name: "animal",
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
        }
    )
}
