use dioxus::prelude::*;
use dioxus_components::components::composites::progressbar::*;
use dioxus_components::{attributes::*, components::atoms::button::*};

pub fn ProgressBarPage() -> Element {
    let mut progress = use_signal(|| 45);
    let button_plus_closure = move |_| {
        if progress() < 100 {
            progress += 1;
        }
    };
    let button_minus_closure = move |_| {
        if progress() > 0 {
            progress -= 1;
        }
    };

    rsx!(
        div { class: "flex flex-col space-y-4",
            "PROGRESS BAR"
            ProgressTrack { color: Color::Primary }
            ProgressTrack { color: Color::Secondary }
            ProgressTrack { color: Color::Destructive }
            ProgressTrack { size: Size::Md }
            ProgressTrack { size: Size::Sm }
            ProgressTrack { size: Size::Lg }
            ProgressTrack { size: Size::Xl }
            div {
                Button { onclick: button_plus_closure, variant: ButtonVariant::Outline, "+" }
                Button { onclick: button_minus_closure, variant: ButtonVariant::Outline, "-" }
            }
            ProgressTrack {
                ProgressBar { progress: progress() }
            }
            ProgressTrack {
                ProgressBar { progress: progress(), color: Color::Secondary }
            }
            ProgressTrack {
                ProgressBar { progress: progress(), color: Color::Destructive }
            }
            ProgressTrack {
                ProgressBar { progress: progress(),
                    ProgressLabel { progress: progress() }
                }
            }
            ProgressTrack {
                ProgressBar { progress: progress(), color: Color::Destructive,
                    ProgressLabel { progress: progress() }
                }
            }
            ProgressTrack {
                ProgressBar { progress: progress(),
                    ProgressLabel { class: "font-bold text-black", progress: progress(), show_percentage: false }
                }
            }
        }
    )
}
