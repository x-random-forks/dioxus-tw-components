use tailwind_fuse::*;

def_class_no_variant!(
    BreadcrumbClass,
    r#"flex flex-wrap items-center font-normal gap-2 text-sm text-muted-foreground"#
);

def_class_no_variant!(BreadcrumbItemClass, r#"font-normal last:text-foreground"#);

def_class_no_variant!(BreadcrumbSeparatorClass, r#"font-semibold"#);
