use tailwind_fuse::*;

#[derive(TwClass, PartialEq)]
#[tw(class = "px-2 py-4 h-screen overflow-y-auto scrollbar")]
pub struct ScrollableClass {}
