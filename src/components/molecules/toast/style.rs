use super::props::*;
use crate::attributes::*;

impl Class for ToasterProps {
    fn base(&self) -> &'static str {
        "fixed z-50 w-full md:m-4 md:max-w-[400px] bottom-0 right-0"
    }
}

impl Class for Toast {
    fn base(&self) -> &'static str {
        "bg-background text-foreground text-sm border border-border shadow-global-shadow p-4 m-2 rounded-global-radius relative group"
    }

    // Color is not a ReadOnlySignal so no need to read()
    fn color(&self) -> Option<&'static str> {
        Some(match self.color {
            Color::Primary => "bg-primary text-primary-foreground",
            Color::Secondary => "bg-secondary text-secondary-foreground",
            Color::Destructive => "bg-destructive text-destructive-foreground",
            Color::Success => "bg-success text-success-foreground",
            _ => "bg-background text-foreground",
        })
    }
}

// impl std::fmt::Display for ToastPosition {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 ToastPosition::TopRight => "top-0 right-0",
//                 ToastPosition::TopLeft => "top-0 left-0",
//                 ToastPosition::BottomRight => "bottom-0 right-0",
//                 ToastPosition::BottomLeft => "bottom-0 left-0",
//             }
//         )
//     }
// }
