use errors::*;
use super::element_format::ElementFormat;

pub trait VariableFormat {
    type Extra: Default;

    fn format<E>(&self, out: &mut E, depth: usize, extra: &mut Self::Extra) -> Result<()>
        where E: ElementFormat;
}
