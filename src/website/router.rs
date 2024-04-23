use dioxus::prelude::*;

use super::pages::{components::sidenav::SideNavComp, header::Header};
use crate::website::app::HomePage;
use crate::website::pages::components::atoms::*;
use crate::website::pages::components::composites::*;
use crate::website::pages::components::molecule::form::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        HomePage {},
        #[layout(SideNavComp)]
        // Atoms
            #[route("/component/atom/button")]
            ButtonPage {},
            #[route("/component/atom/icon")]
            IconPage {},
            #[route("/component/atom/placeholder")]
            PlaceholderPage {},
            #[route("/component/atom/separator")]
            SeparatorPage {},
            #[route("/component/atom/typography")]
            TypographyPage {},

        // Composites
            #[route("/component/composite/accordion")]
            AccordionPage {},
            #[route("/component/composite/breadcrumb")]
            BreadcrumbPage {},
            #[route("/component/composite/carousel")]
            CarouselPage {},
            #[route("/component/composite/dropdown")]
            DropdownPage {},
            #[route("/component/composite/lightswitch")]
            LightSwitchPage {},
            #[route("/component/composite/modal")]
            ModalPage {},
            #[route("/component/composite/navbar")]
            NavbarPage {},
            #[route("/component/composite/progressbar")]
            ProgressBarPage {},
            #[route("/component/composite/scrollable")]
            ScrollablePage {},
            #[route("/component/composite/table")]
            TablePage {},
            #[route("/component/composite/tabs")]
            TabsPage {},

        // Form
            #[route("/component/molecule/form/checkbox")]
            CheckboxPage {},
            #[route("/component/molecule/form/formlist")]
            FormListPage {},
            #[route("/component/molecule/form/input")]
            InputPage {},
            #[route("/component/molecule/form/radiogroup")]
            RadioGroupPage {},
            #[route("/component/molecule/form/select")]
            SelectPage {},
            #[route("/component/molecule/form/slider")]
            SliderPage {},
            #[route("/component/molecule/form/textarea")]
            TextAreaPage {},
            #[route("/component/molecule/form/toggle")]
            TogglePage {},
            #[end_layout]
        #[route("/..route")]
        NotFound {}
}

fn NotFound() -> Element {
    rsx!( div { "404 Not Found" } )
}
