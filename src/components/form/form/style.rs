use super::props::*;
use crate::types::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum FormFlow {
    #[default]
    Vertical,
    Horizontal,
}

impl Variation for FormProps {
    fn variant(&self) -> &'static str {
        match self.flow {
            FormFlow::Vertical => todo!(),
            FormFlow::Horizontal => todo!(),
        }
    }
}
