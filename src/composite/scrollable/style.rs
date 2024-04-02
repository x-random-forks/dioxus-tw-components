use super::ScrollableProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<ScrollableProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "h-screen overflow-y-auto scrollbar",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
