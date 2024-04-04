use dioxus::prelude::*;
use dioxus_components_bin::composite::breadcrumb::*;

pub fn BreadcrumbPage() -> Element {
    rsx!(
        div { class: "",
            "BREADCRUMB PAGE"
            Breadcrumb {  class:"text-sm",
                BreadcrumbItem { "Home" }
                BreadcrumbSeparator {}
                BreadcrumbItem { "Library" }
                BreadcrumbSeparator {}
                BreadcrumbItem { "Data" }
            }
            Breadcrumb {
                BreadcrumbItem { "Home" }
                BreadcrumbSeparator { "/" }
                BreadcrumbItem { "Library" }
                BreadcrumbSeparator { "/" }
                BreadcrumbItem { "Data" }
            }
        }
    )
}
