use super::props::*;
use crate::attributes::*;

impl BaseClass for ToastListProps {
    fn base(&self) -> &'static str {
        "fixed z-50 w-full md:m-4 md:max-w-[400px]"
    }
}

impl BaseClass for Toast {
    fn base(&self) -> &'static str {
        "p-4 m-2 rounded-global-radius"
    }
}

impl Colorable for Toast {
    fn color(&self) -> &'static str {
        match self.color {
            Color::Primary => "bg-primary text-primary-foreground",
            Color::Secondary => "bg-secondary text-secondary-foreground",
            Color::Destructive => "bg-destructive text-destructive-foreground",
            Color::Success => "bg-success text-success-foreground",
            Color::Default | _ => "bg-background text-foreground",
        }
    }
}

impl std::fmt::Display for ToastPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastPosition::TopRight => "top-0 right-0",
                ToastPosition::TopLeft => "top-0 left-0",
                ToastPosition::BottomRight => "bottom-0 right-0",
                ToastPosition::BottomLeft => "bottom-0 left-0",
            }
        )
    }
}
