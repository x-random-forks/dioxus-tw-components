use dioxus::prelude::*;
use dioxus_components_bin::test_comp::*;

pub fn TestPage() -> Element {
    rsx!(
        "My Test Page"
        div { class: "flex flex-col space-y-2 font-bold border border-black",
            div { class: "flex items-center",
                TestText { variant: TestTextVariant::Italic, color: Color::Default }
                TestBox { color: Color::Default }
            }
            div { class: "flex items-center",
                TestText { variant: TestTextVariant::Underline, color: Color::Primary }
                TestBox { color: Color::Primary }
            }
            div { class: "flex items-center",
                TestText { color: Color::Secondary }
                TestBox { variant: TestBoxVariant::Rounded, color: Color::Secondary }
            }
            div { class: "flex items-center",
                TestText { color: Color::Destructive }
                TestBox {  variant: TestBoxVariant::Circle, color: Color::Destructive }
            }
            div { class: "flex items-center",
                TestText { color: Color::Accent }
                TestBox { color: Color::Accent }
            }
            div { class: "flex items-center",
                TestText { color: Color::Muted }
                TestBox { color: Color::Muted }
            }
        }
    )
}
