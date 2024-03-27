use super::FormRangeProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<FormRangeProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "w-full accent-foreground",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
