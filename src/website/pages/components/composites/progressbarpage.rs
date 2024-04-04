use dioxus::prelude::*;
use dioxus_components_bin::{atom::button::*, composite::progressbar::*};

pub fn ProgressBarPage() -> Element {
    let mut progress = use_signal(|| 45);
    let button_plus_closure = move |_| progress.set(progress() + 1);
    let button_minus_closure = move |_| progress.set(progress() - 1);

    rsx!(
        div { class: "flex flex-col space-y-4",
            "PROGRESS BAR"
            ProgressTrack { color: ProgressTrackColor::Primary }
            ProgressTrack { color: ProgressTrackColor::Secondary }
            ProgressTrack { color: ProgressTrackColor::Accent }
            ProgressTrack { size: ProgressTrackSize::Md }
            ProgressTrack { size: ProgressTrackSize::Sm }
            ProgressTrack { size: ProgressTrackSize::Lg }
            ProgressTrack { size: ProgressTrackSize::Xl }
            div {
                Button { onclick: button_plus_closure, "+" }
                Button { onclick: button_minus_closure, "-" }
            }
            ProgressTrack { ProgressBar { progress: progress() } }
            ProgressTrack { ProgressBar { progress: progress(), color: ProgressBarColor::Secondary } }
            ProgressTrack { ProgressBar { progress: progress(), color: ProgressBarColor::Accent } }
        }
    )
}
