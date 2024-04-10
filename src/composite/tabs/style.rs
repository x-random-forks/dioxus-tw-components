use tailwind_fuse::*;

def_class_no_variant!(TabsClass, r#""#);

def_class_no_variant!(
    TabsListClass,
    r#"w-full inline-flex h-8 p-1 items-center justify-center rounded-global-radius bg-muted text-muted-foreground"#
);

def_class_no_variant!(
    TabsTriggerClass,
    r#"inline-flex grow items-center justify-center whitespace-nowrap rounded-global-radius px-2 py-0.5 text-sm font-semibold ring-offset-background transition-all duration-75
    data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow"#
);

def_class_no_variant!(
    TabsContentClass,
    r#"mt-2 border border-border rounded-global-radius shadow"#
);
