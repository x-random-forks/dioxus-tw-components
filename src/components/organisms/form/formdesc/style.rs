use super::props::*;
use crate::attributes::*;

impl BaseClass for FormDescProps {
    fn base(&self) -> &'static str {
        "font-medium text-foreground/80 ml-2"
    }
}
