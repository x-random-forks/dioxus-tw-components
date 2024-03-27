use super::{SelectGroupProps, SelectItemProps, SelectLabelProps, SelectPlaceholderProps};
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<SelectGroupProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "flex flex-col w-full bg-input",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for BaseClass<SelectPlaceholderProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            // This hides the placeholder in the dropdown menu
            BaseClass::Default => "hidden",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for BaseClass<SelectLabelProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "px-2 py-1.5 text-sm font-semibold",
            _ => "",
        };
        write!(f, "{}", class)
    }
}

impl std::fmt::Display for BaseClass<SelectItemProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "flex py-1.5 pl-2 pr-8 text-sm",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
