use dioxus::prelude::*;
use dioxus_components_bin::composite::accordion::*;

pub fn AccordionPage() -> Element {
    rsx!(
        div { class: "",
            "ACCORDION PAGE"
            Accordion { class: "border border-black",
                AccordionItem {
                    AccordionTrigger { id: "t-1", "Trigger 1" }
                    AccordionContent { id: "t-1", class: "", "Content 1" }
                }
                AccordionItem {
                    AccordionTrigger { id: "t-2", "Trigger 2" }
                    AccordionContent { id: "t-2", class: "", "Content 2" }
                }
                AccordionItem {
                    AccordionTrigger { id: "t-3", "Trigger 3" }
                    AccordionContent { id: "t-3", class: "", "Content 3" }
                }
            }
            Accordion { class: "border border-black", multi_open: true,
                AccordionItem {
                    AccordionTrigger { id: "t-4", "Trigger 4", is_open: true}
                    AccordionContent { id: "t-4", class: "", "Content 4" }
                }
                AccordionItem {
                    AccordionTrigger { id: "t-5", "Trigger 5" }
                    AccordionContent { id: "t-5", class: "", "Content 5" }
                }
                AccordionItem {
                    AccordionTrigger { id: "t-6", "Trigger 6", is_open: true}
                    AccordionContent { id: "t-6", class: "", "Content 6" }
                }
            }
        }
    )
}
