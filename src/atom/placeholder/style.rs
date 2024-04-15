use super::props::*;
use crate::types::*;

impl BaseClass for PlaceholderProps {
    fn base(&self) -> &'static str {
        "bg-muted"
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum PlaceholderRadius {
    #[default]
    Global,
    Full,
    None,
}

impl Variation for PlaceholderProps {
    fn variant(&self) -> &'static str {
        match self.radius {
            PlaceholderRadius::Global => "rounded-global-radius",
            PlaceholderRadius::Full => "rounded-full",
            PlaceholderRadius::None => "rounded-none",
        }
    }
}
