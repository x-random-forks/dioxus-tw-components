use super::NavbarProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<NavbarProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "container flex h-12 max-w-screen-2xl items-center",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
