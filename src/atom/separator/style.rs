use super::SeparatorProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<SeparatorProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "bg-border shrink-0",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
