use super::props::*;
use crate::attributes::*;

impl BaseClass for FormLabelProps {
    fn base(&self) -> &'static str {
        "font-normal peer-disabled:opacity-50 peer-disabled:cursor-not-allowed select-none"
    }
}
