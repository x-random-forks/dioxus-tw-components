use dioxus::prelude::*;
use dioxus_components_bin::{atom::checkbox::*, types::Color};

pub fn CheckboxPage() -> Element {
    rsx!(
        "CHECKBOX PAGE"
        Checkbox { checked: true, color: Color::Primary, "Primary" }
        Checkbox { checked: true, color: Color::Secondary, "Secondary" }
        Checkbox { checked: true, color: Color::Destructive, "Destructive" }
        Checkbox { checked: true, color: Color::Success, "Success" }
        Checkbox { checked: true, disabled: true, color: Color::Primary, "Disabled" }
    )
}
