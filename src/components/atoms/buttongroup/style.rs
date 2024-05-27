use super::props::*;
use crate::attributes::*;

impl BaseClass for ButtonGroupProps {
    fn base(&self) -> &'static str {
        "inline-flex border border-border bg-background rounded-global-radius divide-border divide-x"
    }
}

impl BaseClass for ButtonGroupItemProps {
    fn base(&self) -> &'static str {
        "px-4 py-2 font-medium text-foreground first:rounded-l-global-radius last:rounded-r-global-radius hover:bg-foreground/20 active:bg-foreground/30"
    }
}