use super::props::*;
use crate::attributes::*;

impl BaseClass for LabelProps {
    fn base(&self) -> &'static str {
        "text-sm font-medium peer-disabled:text-muted"
    }
}
