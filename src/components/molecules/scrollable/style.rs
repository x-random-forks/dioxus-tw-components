use super::props::*;
use crate::attributes::*;

impl Class for ScrollableProps {
    fn base(&self) -> &'static str {
        "px-extrasmall py-small scrollbar"
    }

    fn orientation(&self) -> Option<&'static str> {
        Some(match self.orientation {
            Orientation::Horizontal => {
                "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90"
            }
            Orientation::Vertical => "overflow-y-auto overflow-x-hidden grid-flow-row",
        })
    }
}
