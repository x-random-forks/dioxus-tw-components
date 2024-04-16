use dioxus::prelude::*;
use dioxus_components_bin::composite::accordion::*;

pub fn AccordionPage() -> Element {
    rsx!(
        div { class: "",
            "ACCORDION PAGE"
            // Accordion {
            //     AccordionItem {
            //         AccordionTrigger { "Trigger 1" }
            //         AccordionContent { "Content 1" }
            //     }
            // }
            Accordion { 
                AccordionItem { 
                    AccordionTrigger { "Trigger 1" }
                    AccordionContent { class: "", "Content 1" }
                }
                AccordionItem { 
                    AccordionTrigger { "Trigger 2" }
                    AccordionContent { class: "", "Content 2" }
                }
                AccordionItem { 
                    AccordionTrigger { "Trigger 3" }
                    AccordionContent { class: "", "Content 3" }
                }
            }
        }
    )
}
