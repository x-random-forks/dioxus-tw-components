use dioxus::prelude::*;
use dioxus_components_bin::{atom::label::Label, composite::radiogroup::*};

pub fn RadioGroupPage() -> Element {
    rsx!(
        div { class: "flex flex-col gap-2 text-base",
            "RADIO GROUP PAGE"
            RadioGroup { name: "gender", default_value: "male",
                Label { r#for: "gender", "Choose birth gender" }
                RadioItem { value: "male", name: "gender", required: true, "Male" }
                RadioItem { value: "female", name: "gender", "Female" }
                RadioItem { value: "other", name: "gender", "Other" }
                RadioItem { value: "disabled", name: "gender", disabled: true, "Disabled" }
            }
        }
    )
}
