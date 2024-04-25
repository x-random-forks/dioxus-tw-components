use super::props::*;
use crate::attributes::*;

impl BaseClass for BreadcrumbProps {
    fn base(&self) -> &'static str {
        "flex flex-row items-center font-normal gap-2 d-text-small text-muted-foreground"
    }
}

impl BaseClass for BreadcrumbItemProps {
    fn base(&self) -> &'static str {
        "font-normal last:text-foreground"
    }
}

impl BaseClass for BreadcrumbSeparatorProps {
    fn base(&self) -> &'static str {
        "font-semibold"
    }
}
