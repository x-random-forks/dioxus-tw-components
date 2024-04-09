use tailwind_fuse::*;

// Check input.css to personalize scrollbar class
#[derive(TwClass, PartialEq)]
#[tw(class = "px-2 py-4 scrollbar")]
pub struct ScrollableClass {
    horizontal: ScrollableHorizontal,
}

#[derive(TwVariant, PartialEq)]
pub enum ScrollableHorizontal {
    #[tw(default, class = "overflow-y-auto overflow-x-hidden grid-flow-row")]
    Vertical,
    // TODO WIP Trying to make horizontal scroll with mouse wheel
    #[tw(class = "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90")]
    Horizontal,
}

impl From<bool> for ScrollableHorizontal {
    fn from(horizontal: bool) -> Self {
        match horizontal {
            true => ScrollableHorizontal::Horizontal,
            false => ScrollableHorizontal::Vertical,
        }
    }
}
