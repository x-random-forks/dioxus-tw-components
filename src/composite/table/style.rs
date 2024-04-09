use tailwind_fuse::*;

class!(TableClass, "w-full caption-bottom text-sm");

class!(TableHeaderClass, "[&_tr]:border-b");

class!(TableBodyClass, "[&_tr:last-child]:border-0");

class!(
    TableFooterClass,
    "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 hover:bg-muted/70"
);

class!(
    TableHeadClass,
    "h-10 px-2 text-left align-middle font-medium text-muted-foreground"
);

class!(
    TableRowClass,
    "border-b transition-colors hover:bg-muted/50"
);

class!(TableCellClass, "p-2 align-middle");

class!(TableCaptionClass, "mt-4 text-sm text-muted-foreground");
