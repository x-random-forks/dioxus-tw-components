use dioxus::prelude::*;

use super::pages::{components::sidenav::SideNavComp, header::Header};
use crate::website::app::HomePage;
use crate::website::pages::components::atoms::*;
use crate::website::pages::components::composites::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        HomePage {},
        #[layout(SideNavComp)]
            #[route("/component/atom/button")]
            ButtonPage {},
            #[route("/component/atom/checkbox")]
            CheckboxPage {},
            #[route("/component/atom/slider")]
            SliderPage {},
            #[route("/component/atom/icon")]
            IconPage {},
            #[route("/component/atom/input")]
            InputPage {},
            #[route("/component/atom/placeholder")]
            PlaceholderPage {},
            #[route("/component/atom/separator")]
            SeparatorPage {},
            #[route("/component/atom/textarea")]
            TextAreaPage {},
            #[route("/component/atom/toggle")]
            TogglePage {},
            #[route("/component/atom/typography")]
            TypographyPage {},
            #[route("/component/composite/accordion")]
            AccordionPage {},
            #[route("/component/composite/breadcrumb")]
            BreadcrumbPage {},
            #[route("/component/composite/dropdown")]
            DropdownPage {},
            #[route("/component/composite/formlist")]
            FormListPage {},
            #[route("/component/composite/lightswitch")]
            LightSwitchPage {},
            #[route("/component/composite/modal")]
            ModalPage {},
            #[route("/component/composite/navbar")]
            NavbarPage {},
            #[route("/component/composite/progressbar")]
            ProgressBarPage {},
            #[route("/component/composite/radiogroup")]
            RadioGroupPage {},
            #[route("/component/composite/scrollable")]
            ScrollablePage {},
            #[route("/component/composite/select")]
            SelectPage {},
            #[route("/component/composite/table")]
            TablePage {},
            #[route("/component/composite/tabs")]
            TabsPage {},
        #[end_layout]
    #[route("/..route")]
    NotFound {}
}

fn NotFound() -> Element {
    rsx!( div { "404 Not Found" } )
}
