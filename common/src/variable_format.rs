use super::*;

pub trait VariableFormat {
    type Extra: Default;

    fn format<E>(&self, out: &mut E, depth: usize, extra: &mut Self::Extra) -> Result<()>
    where
        E: ElementFormat;
}
