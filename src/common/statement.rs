use errors::*;
use common::ElementFormat;
use super::element_spec::ElementSpec;
use super::variable_format::VariableFormat;

/// A single statement, made up by variables.
#[derive(Debug, Clone)]
pub struct Statement<Var>
    where Var: VariableFormat
{
    pub parts: Vec<Var>,
}

impl<Var> Statement<Var>
    where Var: VariableFormat
{
    pub fn new() -> Statement<Var> {
        Statement { parts: Vec::new() }
    }

    pub fn push<V>(&mut self, variable: V)
        where V: Into<Var>
    {
        self.parts.push(variable.into());
    }

    pub fn join<A>(self, separator: A) -> Statement<Var>
        where A: Into<Var> + Clone
    {
        let mut it = self.parts.into_iter();

        let part: Var = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<Var> = Vec::new();
        parts.push(part);

        while let Some(part) = it.next() {
            parts.push(separator.clone().into());
            parts.push(part);
        }

        Statement { parts: parts }
    }

    pub fn format<E>(&self, out: &mut E) -> Result<()>
        where E: ElementFormat
    {
        for part in &self.parts {
            part.format(out)?;
        }

        Ok(())
    }
}

impl<'a, T, Var> From<&'a T> for Statement<Var>
    where T: Into<Statement<Var>> + Clone,
          Var: VariableFormat
{
    fn from(value: &'a T) -> Statement<Var> {
        value.clone().into()
    }
}

impl<Var> From<Statement<Var>> for ElementSpec<Var>
    where Var: VariableFormat
{
    fn from(value: Statement<Var>) -> ElementSpec<Var> {
        ElementSpec::Statement(value)
    }
}
