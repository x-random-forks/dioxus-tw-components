use dioxus::prelude::*;
use dioxus_components_bin::components::composites::table::*;

pub fn TablePage() -> Element {
    rsx!(
        "TABLE PAGE"
        div { class: "relative w-full", Table { 
            TableCaption { "This is a table caption" }
            TableHeader { TableRow { 
                TableHead { "Header 1" }
                TableHead { "Header 2" }
                TableHead { "Header 3" }
            } }
            TableBody { 
                TableRow { 
                    TableCell { "Row 1, Cell 1" }
                    TableCell { "Row 1, Cell 2" }
                    TableCell { "Row 1, Cell 3" }
                }
                TableRow { 
                    TableCell { "Row 2, Cell 1" }
                    TableCell { "Row 2, Cell 2" }
                    TableCell { "Row 2, Cell 3" }
                }
                TableRow { 
                    TableCell { "Row 3, Cell 1" }
                    TableCell { "Row 3, Cell 2" }
                    TableCell { "Row 3, Cell 3" }
                }
            }
            TableFooter { TableRow { 
                TableCell { "Footer 1" }
                TableCell { "Footer 2" }
                TableCell { "Footer 3" }
            } }
        } }
    )
}
