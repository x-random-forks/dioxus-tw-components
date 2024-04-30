use super::props::*;
use crate::attributes::*;

impl BaseClass for FormProps {
    fn base(&self) -> &'static str {
        "grid grid-cols-1 lg:grid-cols-2 space-x-4"
    }
}

impl BaseClass for FormHeaderProps {
    fn base(&self) -> &'static str {
        "w-full lg:col-span-2 border-b pb-2 flex flex-col items-center justify-center"
    }
}

impl BaseClass for FormFooterProps {
    fn base(&self) -> &'static str {
        "mt-2 lg:col-span-2 flex flex-col items-center justify-center"
    }
}
