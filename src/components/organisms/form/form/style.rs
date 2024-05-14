use super::props::*;
use crate::attributes::*;

impl BaseClass for FormProps {
    fn base(&self) -> &'static str {
        "grid grid-cols-1 space-x-4 max-w-screen-md"
    }
}

impl BaseClass for FormHeaderProps {
    fn base(&self) -> &'static str {
        "w-full border-b pb-2 flex flex-col items-center justify-center"
    }
}

impl BaseClass for FormFooterProps {
    fn base(&self) -> &'static str {
        "mt-2 flex flex-col items-center justify-center"
    }
}

impl BaseClass for FormTitleProps {
    fn base(&self) -> &'static str {
        "font-medium text-foreground"
    }
}

impl BaseClass for FormDescProps {
    fn base(&self) -> &'static str {
        "span text-sm italic"
    }
}


impl BaseClass for FormLabelProps {
    fn base(&self) -> &'static str {
        "font-normal peer-disabled:opacity-50 peer-disabled:cursor-not-allowed select-none"
    }
}

impl BaseClass for FormChildProps {
    fn base(&self) -> &'static str {
        "flex flex-col space-y-1 my-4"
    }
}