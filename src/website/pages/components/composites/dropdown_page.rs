use dioxus::prelude::*;
use dioxus_components_bin::{
    components::atoms::{button::*, separator::Separator},
    components::composites::dropdown::*,
};

pub fn DropdownPage() -> Element {
    rsx!(
        div { class: "flex gap-4",
            "DROPDOWN PAGE"
            div {
                Dropdown { closing_delay_ms: 1000,
                    DropdownToggle { 
                        Button { variant: ButtonVariant::Outline, "Dropdown" }
                    }
                    DropdownContent { 
                        div { class: "paragraph flex flex-col space-y-2",
                            a {
                                class: "anchor",
                                href: "http://localhost:8080/",
                                "link"
                            }
                            div { "content" }
                            Separator {}
                            div { "very long long long long content" }
                        }
                    }
                }
                Dropdown { 
                    DropdownToggle { 
                        Button { variant: ButtonVariant::Outline, "Dropdown" }
                    }
                    DropdownContent { 
                        div { class: "paragraph flex flex-col space-y-2",
                            div { "content" }
                            Separator {}
                            div { "very long long long long content" }
                        }
                    }
                }
            }
        }
    )
}
