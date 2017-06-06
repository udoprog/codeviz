use errors::*;
use super::element_format::ElementFormat;

pub trait VariableFormat {
    fn format<E>(&self, out: &mut E) -> Result<()> where E: ElementFormat;
}
