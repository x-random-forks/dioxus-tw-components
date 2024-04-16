use super::props::*;
use crate::types::*;

impl BaseClass for SliderProps {
    fn base(&self) -> &'static str {
        "w-full accent-foreground"
    }
}
