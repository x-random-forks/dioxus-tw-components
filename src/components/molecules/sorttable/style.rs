use super::props::*;
use crate::attributes::*;

impl<T: 'static + std::clone::Clone + std::cmp::PartialEq + ToTableData> Class
    for SortTableProps<T>
{
    fn base(&self) -> &'static str {
        "w-full caption-bottom text-sm text-foreground bg-background"
    }
}
