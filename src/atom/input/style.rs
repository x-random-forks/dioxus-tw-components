use super::props::*;
use crate::types::*;

impl BaseClass for InputProps {
    fn base(&self) -> &'static str {
        "flex w-full px-3 py-1 text-foreground bg-background border border-input rounded-global-radius hover:brightness-105 focus:outline-none focus:brightness-105 focus:ring-ring focus:ring-offset-2 focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100 file:font-medium file:bg-input file:rounded-sm file:border-0 file:items-center file:justify-center"
    }
}

impl Sizable for InputProps {
    fn size(&self) -> &'static str {
        match self.size {
            Size::Xs => "h-4 text-xs",
            Size::Sm => "h-6 text-xs",
            Size::Md => "h-8 text-sm",
            Size::Lg => "h-10 text-base",
            Size::Xl => "h-12 text-lg",
        }
    }
}
