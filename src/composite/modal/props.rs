use std::{ops::Deref, rc::Rc, sync::Arc};

use super::style::*;
use crate::{atom::button::*, Component};
use component_derive::Component;
use dioxus::{
    html::{
        button,
        geometry::euclid::{Point2D, Rect, Size2D},
    },
    prelude::*,
};
use tailwind_fuse::*;

#[derive(Props, Clone, PartialEq, Component)]
pub struct ModalProps {
    children: Element,
    // Styling
    #[props(default)]
    class: String,
}

impl Component for ModalProps {
    fn view(self) -> Element {
        let mut sig = use_signal(|| false);
        let mut elem = use_signal(Vec::<Rc<MountedData>>::new);
        // TODO
        // let mut elemsingle = use_signal(|| Rc::<MountedData>::new);

        let rect: Rect<f64, f64> = Rect::zero();
        let mut rect_modal = use_signal(|| rect);

        // let mut future = use_resource(move || async move {
        //     if let Some(elem) = elem.last() {
        //         let rec = elem.get_client_rect().await.unwrap();
        //         log::debug!("elem: {:#?}", rec);

        //         rect_modal.set(rec);
        //     }
        // });

        let button_closure = move |_| {
            sig.toggle();
            log::debug!("sig: {:#?}", sig());
        };

        let modal_closure = move |event: Event<MouseData>| {
            sig.toggle();
            // let point = get_click_position(event);
            // log::debug!("point: {:#?}", point);
            // log::debug!("rec_sig: {:#?}", rect_modal());

            // if rect_modal().contains(point) {
            //     log::debug!("INSIDE");
            // } else {
            //     sig.toggle();
            //     log::debug!("OUTSIDE");
            // }
        };

        rsx!(
            button { class: "border-4 border-black", onclick: button_closure, "Open modal" }
            if sig() {
                div { class: "fixed top-0 left-0 w-full h-full z-40 bg-[linear-gradient(_45deg,magenta,rebeccapurple,dodgerblue,green_)] opacity-75",
                    onclick: modal_closure,
                }
                div {
                    // onmounted: move |cx| {
                    //     log::debug!("MOUNTED");
                    //     elem.push(cx.data());

                    //     let _ = use_resource(move || async move {
                    //         if let Some(elem) = elem.last() {
                    //             let rec = elem.get_client_rect().await.unwrap();
                    //             log::debug!("elem: {:#?}", rec);

                    //             rect_modal.set(rec);
                    //         }
                    //     });
                    // },
                    class: "fixed top-[50%] left-[50%] z-50 border-2 border-black bg-background",
                    "LONG LONG LONG LONG LONG LONG LONG LONG CONTENT"
                    button { class: "border-4 border-black", onclick: button_closure, "CLOSE ME" }
                }
            }
        )
    }
}

fn get_click_position(event: Event<MouseData>) -> Point2D<f64, f64> {
    event.deref().client_coordinates().cast_unit()
}
