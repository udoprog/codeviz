use errors::*;
use super::element_format::ElementFormat;

pub trait VariableFormat {
    fn format<E>(&self, out: &mut E, depth: usize) -> Result<()> where E: ElementFormat;
}
