use super::props::*;
use crate::attributes::*;

impl BaseClass for ScrollableProps {
    fn base(&self) -> &'static str {
        "px-extrasmall py-small scrollbar"
    }
}

impl Orientable for ScrollableProps {
    fn orientation(&self) -> &'static str {
        match self.orientation {
            Orientation::Horizontal => {
                "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90"
            }
            Orientation::Vertical => "overflow-y-auto overflow-x-hidden grid-flow-row",
        }
    }
}
