use dioxus::prelude::*;
use dioxus_components_bin::composite::tabs::*;

pub fn TabsPage() -> Element {
    rsx!(
        "TABS PAGE"
        Tabs { default_tab: "tab-1", class: "text-xl w-96",
            TabsList { 
                TabsTrigger { id: "tab-1", "Tab 1" }
                TabsTrigger { id: "tab-2", "Tab 2" }
                TabsTrigger { id: "tab-3", "Tab 3" }
            }
            TabsContent { id: "tab-1", "Tab 1 Content" }
            TabsContent { id: "tab-2", "Tab 2 Content" }
            TabsContent { id: "tab-3", "Tab 3 Content" }
        }
    )
}
