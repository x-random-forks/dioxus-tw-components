use super::props::*;
use crate::attributes::*;

impl BaseClass for TableProps {
    fn base(&self) -> &'static str {
        "w-full caption-bottom text-sm"
    }
}

impl BaseClass for TableHeaderProps {
    fn base(&self) -> &'static str {
        "[&_tr]:border-b"
    }
}

impl BaseClass for TableBodyProps {
    fn base(&self) -> &'static str {
        "[&_tr:last-child]:border-0"
    }
}

impl BaseClass for TableFooterProps {
    fn base(&self) -> &'static str {
        "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 hover:bg-muted/70"
    }
}

impl BaseClass for TableHeadProps {
    fn base(&self) -> &'static str {
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground"
    }
}

impl BaseClass for TableRowProps {
    fn base(&self) -> &'static str {
        "border-b transition-colors hover:bg-muted/50"
    }
}

impl BaseClass for TableCellProps {
    fn base(&self) -> &'static str {
        "p-2 align-middle"
    }
}

impl BaseClass for TableCaptionProps {
    fn base(&self) -> &'static str {
        "mt-4 text-sm text-muted-foreground"
    }
}
