use super::props::*;
use crate::attributes::*;

impl BaseClass for LabelProps {
    fn base(&self) -> &'static str {
        "text-sm font-medium select-none peer-disabled:opacity-40 peer-disabled:font-normal"
    }
}
