use dioxus::prelude::*;

use super::pages::layouts::sidenav::SideNavLayouts;
use super::pages::{components::sidenav::SideNavComp, header::Header};
use crate::website::app::HomePage;
use crate::website::pages::components::atoms::*;
use crate::website::pages::components::composites::*;
use crate::website::pages::components::molecule::form::*;
use crate::website::pages::layouts::tmp_page::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        HomePage {},
        #[layout(SideNavComp)]
        // Atoms
            #[route("/components/atoms/button")]
            ButtonPage {},
            #[route("/components/atoms/icon")]
            IconPage {},
            #[route("/components/atoms/placeholder")]
            PlaceholderPage {},
            #[route("/components/atoms/separator")]
            SeparatorPage {},
            #[route("/components/atoms/typography")]
            TypographyPage {},

        // Composites
            #[route("/components/composites/accordion")]
            AccordionPage {},
            #[route("/components/composites/breadcrumb")]
            BreadcrumbPage {},
            #[route("/components/composites/carousel")]
            CarouselPage {},
            #[route("/components/composites/dropdown")]
            DropdownPage {},
            #[route("/components/composites/lightswitch")]
            LightSwitchPage {},
            #[route("/components/composites/modal")]
            ModalPage {},
            #[route("/components/composites/navbar")]
            NavbarPage {},
            #[route("/components/composites/progressbar")]
            ProgressBarPage {},
            #[route("/components/composites/scrollable")]
            ScrollablePage {},
            #[route("/components/composites/table")]
            TablePage {},
            #[route("/components/composites/tabs")]
            TabsPage {},

        // Form
            #[route("/components/molecules/form/checkbox")]
            CheckboxPage {},
            #[route("/components/molecules/form/formlist")]
            FormListPage {},
            #[route("/components/molecules/form/input")]
            InputPage {},
            #[route("/components/molecules/form/radiogroup")]
            RadioGroupPage {},
            #[route("/components/molecules/form/select")]
            SelectPage {},
            #[route("/components/molecules/form/slider")]
            SliderPage {},
            #[route("/components/molecules/form/textarea")]
            TextAreaPage {},
            #[route("/components/molecules/form/toggle")]
            TogglePage {},
        #[end_layout]
        
        #[layout(SideNavLayouts)]
            #[route("/layouts/tmp")]
            TmpPage {},

        #[route("/..route")]
        NotFound {}
}

fn NotFound() -> Element {
    rsx!( div { "404 Not Found" } )
}
