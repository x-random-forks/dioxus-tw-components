use dioxus::prelude::*;
use dioxus_components_bin::atom::checkbox::*;

pub fn CheckboxPage() -> Element {
    rsx!(
        div { class: "h-screen",
            "CHECKBOX PAGE"
            Checkbox { checked: true, color: CheckboxColor::Primary, "Primary" }
            Checkbox { checked: true, color: CheckboxColor::Secondary, "Secondary" }
            Checkbox { checked: true, color: CheckboxColor::Primary, "Primary" }
            Checkbox { checked: true, disabled: true, color: CheckboxColor::Primary, "Disabled" }
        }
    )
}
