use super::AnchorProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<AnchorProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => "text-blue-500",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
