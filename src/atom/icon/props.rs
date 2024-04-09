use super::style::IconSvg;
use crate::*;
use component_derive::Component;

pub use IconSvg::*;

#[derive(PartialEq, Props, Clone, Component)]
pub struct IconProps {
    // Namespace of the svg
    #[props(default = "http://www.w3.org/2000/svg".to_string())]
    xmlns: String,
    // Icon type
    #[props(default = IconSvg::HollowCircle)]
    svg: IconSvg,
}

// TODO
impl Component for IconProps {
    fn view(self) -> Element {
        // let fill_color = match self.color {
        //     Color::DefaultColor => "fill-foreground",
        //     Color::Primary => "fill-primary",
        //     Color::Secondary => "fill-secondary",
        //     Color::Accent => "fill-accent",
        //     _ => "fill-none",
        // };
        // let stroke_color = match self.color {
        //     Color::DefaultColor => "stroke-foreground",
        //     Color::Primary => "stroke-primary",
        //     Color::Secondary => "stroke-secondary",
        //     Color::Accent => "stroke-accent",
        //     _ => "stroke-none",
        // };

        let stroke_color = "stroke-foreground";
        let fill_color = "fill-foreground";

        let svg = match self.svg {
            IconSvg::HollowCircle => rsx!(
                circle {
                    class: "{stroke_color} fill-none stroke-1",
                    cx: "12",
                    cy: "12",
                    r: "10"
                }
            ),
            IconSvg::CircleInnerCircle => rsx!(
                circle {
                    class: "{stroke_color} fill-none stroke-1",
                    cx: "12",
                    cy: "12",
                    r: "10"
                }
                circle { class: "{fill_color}", cx: "12", cy: "12", r: "6" }
            ),
            IconSvg::GitHub => rsx!(
                path {
                    class: "{fill_color}",
                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                }
            ),
            IconSvg::Sun => rsx!(
                circle {
                    class: "{stroke_color}",
                    cx: "12",
                    cy: "12",
                    r: "5",
                    stroke_width: "1.5"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M12 2V4"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M12 20V22"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M4 12L2 12"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M22 12L20 12"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M19.7778 4.22266L17.5558 6.25424"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M4.22217 4.22266L6.44418 6.25424"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M6.44434 17.5557L4.22211 19.7779"
                }
                path {
                    class: "{stroke_color}",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    d: "M19.7778 19.7773L17.5558 17.5551"
                }
            ),
            IconSvg::Moon => rsx!(
                path {
                    class: "{fill_color}",
                    d: "M21.0672 11.8568L20.4253 11.469L21.0672 11.8568ZM12.1432 2.93276L11.7553 2.29085V2.29085L12.1432 2.93276ZM21.25 12C21.25 17.1086 17.1086 21.25 12 21.25V22.75C17.9371 22.75 22.75 17.9371 22.75 12H21.25ZM12 21.25C6.89137 21.25 2.75 17.1086 2.75 12H1.25C1.25 17.9371 6.06294 22.75 12 22.75V21.25ZM2.75 12C2.75 6.89137 6.89137 2.75 12 2.75V1.25C6.06294 1.25 1.25 6.06294 1.25 12H2.75ZM15.5 14.25C12.3244 14.25 9.75 11.6756 9.75 8.5H8.25C8.25 12.5041 11.4959 15.75 15.5 15.75V14.25ZM20.4253 11.469C19.4172 13.1373 17.5882 14.25 15.5 14.25V15.75C18.1349 15.75 20.4407 14.3439 21.7092 12.2447L20.4253 11.469ZM9.75 8.5C9.75 6.41182 10.8627 4.5828 12.531 3.57467L11.7553 2.29085C9.65609 3.5593 8.25 5.86509 8.25 8.5H9.75ZM12 2.75C11.9115 2.75 11.8077 2.71008 11.7324 2.63168C11.6686 2.56527 11.6538 2.50244 11.6503 2.47703C11.6461 2.44587 11.6482 2.35557 11.7553 2.29085L12.531 3.57467C13.0342 3.27065 13.196 2.71398 13.1368 2.27627C13.0754 1.82126 12.7166 1.25 12 1.25V2.75ZM21.7092 12.2447C21.6444 12.3518 21.5541 12.3539 21.523 12.3497C21.4976 12.3462 21.4347 12.3314 21.3683 12.2676C21.2899 12.1923 21.25 12.0885 21.25 12H22.75C22.75 11.2834 22.1787 10.9246 21.7237 10.8632C21.286 10.804 20.7293 10.9658 20.4253 11.469L21.7092 12.2447Z"
                }
            )
        };

        rsx!(
            svg {
                xmlns: "{self.xmlns}",
                view_box: "0 0 24 24",
                stroke_linecap: "round",
                {svg}
            }
        )
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#9b1717"
//  stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle"><circle cx="12" cy="12" r="10"/></svg>
