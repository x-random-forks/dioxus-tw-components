use dioxus::prelude::*;
use dioxus_components_bin::components::composites::breadcrumb::*;

pub fn BreadcrumbPage() -> Element {
    rsx!(
        div { class: "",
            "BREADCRUMB PAGE"
            Breadcrumb { class: "",
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
