use super::TextAreaProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<TextAreaProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::Default => {
                "
                flex w-full \
                text-sm text-foreground \
                bg-input focus:brightness-105 hover:brightness-105 \
                px-3 py-2 \
                border border-input \
                rounded-global-radius \
                disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100"
            }
            _ => "",
        };
        write!(f, "{}", class)
    }
}
