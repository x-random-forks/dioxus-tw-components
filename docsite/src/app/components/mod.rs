pub mod atoms;
pub use atoms::{button::ButtonPage, buttongroup::ButtonGroupPage, placeholder::PlaceholderPage};
pub mod layout;
pub use layout::SideBarComponent;

pub mod molecules;
pub use molecules::tabs::TabsPage;

pub mod organisms;
pub use organisms::form::{
    checkbox::CheckboxPage, input::InputPage, radio::RadioPage, select::SelectPage,
    slider::SliderPage, textarea::TextAreaPage, toggle::TogglePage
};

pub mod preview;

use crate::app::router::Route;
use dioxus::prelude::*;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let route: Route = use_route();

    match route {
        Route::ComponentPage { name } => match name.as_str() {
            "button" => rsx!(
                ButtonPage {}
            ),
            "buttongroup" => rsx!(
                ButtonGroupPage {}
            ),
            "placeholder" => rsx!(
                PlaceholderPage {}
            ),
            "tabs" => rsx!(
                TabsPage {}
            ),
            "checkbox" => rsx!(
                CheckboxPage {}
            ),
            "input" => rsx!(
                InputPage {}
            ),
            "radio" => rsx!(
                RadioPage {}
            ),
            "select" => rsx!(
                SelectPage {}
            ),
            "slider" => rsx!(
                SliderPage {}
            ),
            "textarea" => rsx!(
                TextAreaPage {}
            ),
            "toggle" => rsx!(
                TogglePage {}
            ),
            _ => {
                rsx!( "Component not found" )
            }
        },
        _ => {
            rsx!( "How did you got there ?" )
        }
    }
}
