use super::InputProps;
use crate::styling::BaseClass;

impl std::fmt::Display for BaseClass<InputProps> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            BaseClass::BaseClass => {
                "
                flex w-full \
                h-8 \
                px-3 py-2 \
                text-sm text-foreground \
                bg-background \
                border border-input \
                rounded-global-radius \
                hover:brightness-105 \
                focus:outline-none \
                focus:brightness-105 focus:ring-ring focus:ring-offset-2 focus:ring-2 \
                disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100 \
                file:font-medium file:bg-input file:rounded-global-radius file:border-0"
            }
            _ => "",
        };
        write!(f, "{}", class)
    }
}
