use tailwind_fuse::*;

#[derive(TwClass, PartialEq)]
#[tw(class = "h-screen overflow-y-auto scrollbar")]
pub struct ScrollableClass {}
