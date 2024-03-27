use super::ToggleProps;
use crate::styling::{BaseClass, Color, Size};

impl std::fmt::Display for BaseClass<ToggleProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "relative bg-input peer peer-focus:outline-none peer-focus:ring-2 rounded-full peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:bg-white after:border-gray-300 after:border after:rounded-full after:transition-all",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for Color<ToggleProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Color::Primary => "peer-checked:bg-primary peer-focus:ring-foreground",
            Color::Secondary => "peer-checked:bg-secondary peer-focus:ring-foreground",
            Color::Accent => "peer-checked:bg-accent peer-focus:ring-foreground",
            _ => "",
        };
        write!(f, "{}", size)
    }
}

impl std::fmt::Display for Size<ToggleProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Lg | Size::Xl => "w-9 h-5 after:top-[2px] after:start-[2px] after:h-4 after:w-4",
            Size::Sm | Size::Xs => "w-11 h-6 after:top-[2px] after:start-[2px] after:h-5 after:w-5",
            Size::Md | _ => "w-14 h-7 after:top-0.5 after:start-[4px] after:h-6 after:w-6",
        };
        write!(f, "{}", size)
    }
}
