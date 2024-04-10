use tailwind_fuse::*;

def_class_no_variant!(TableClass, "w-full caption-bottom text-sm");

def_class_no_variant!(TableHeaderClass, "[&_tr]:border-b");

def_class_no_variant!(TableBodyClass, "[&_tr:last-child]:border-0");

def_class_no_variant!(
    TableFooterClass,
    "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 hover:bg-muted/70"
);

def_class_no_variant!(
    TableHeadClass,
    "h-10 px-2 text-left align-middle font-medium text-muted-foreground"
);

def_class_no_variant!(
    TableRowClass,
    "border-b transition-colors hover:bg-muted/50"
);

def_class_no_variant!(TableCellClass, "p-2 align-middle");

def_class_no_variant!(TableCaptionClass, "mt-4 text-sm text-muted-foreground");
