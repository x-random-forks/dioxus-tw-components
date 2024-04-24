use dioxus::prelude::*;
use dioxus_components::{
    components::atoms::label::Label, components::molecules::form::radiogroup::*,
};

pub fn RadioGroupPage() -> Element {
    rsx!(
        div { class: "flex flex-col gap-2 text-base",
            "RADIO GROUP PAGE"
            RadioGroup { name: "gender", default_value: "male",
                Label { r#for: "gender", "Choose birth gender" }
                RadioItem { value: "male", name: "gender", "Male" }
                RadioItem { value: "female", name: "gender", "Female" }
                RadioItem { value: "other", name: "gender", "Other" }
            }
        }
    )
}
