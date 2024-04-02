use super::NavbarProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<NavbarProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "flex items-center h-12 border-b border-border",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
