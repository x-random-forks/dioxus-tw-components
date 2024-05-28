use super::props::*;
use crate::attributes::*;

impl BaseClass for NavbarProps {
    fn base(&self) -> &'static str {
        "container flex h-14 max-w-screen-2xl items-center whitespace-nowrap"
    }
}

// TODO add size ?
