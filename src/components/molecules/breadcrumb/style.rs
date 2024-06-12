use super::props::*;
use crate::attributes::*;

impl Class for BreadcrumbProps {
    fn base(&self) -> &'static str {
        "flex flex-row items-center font-normal space-x-2 text-sm text-muted-foreground"
    }
}

impl Class for BreadcrumbItemProps {
    fn base(&self) -> &'static str {
        "font-normal last:text-foreground"
    }
}

impl Class for BreadcrumbSeparatorProps {
    fn base(&self) -> &'static str {
        "font-semibold"
    }
}
