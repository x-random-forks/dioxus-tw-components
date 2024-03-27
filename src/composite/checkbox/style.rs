use super::CheckboxProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<CheckboxProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => "",
            _ => "",
        };
        write!(f, "{}", class)
    }
}
