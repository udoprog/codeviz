use errors::*;
use common::ElementFormat;
use super::variable::Variable;
use super::element_spec::ElementSpec;

/// A single statement, made up by variables.
#[derive(Debug, Clone)]
pub struct Statement {
    pub parts: Vec<Variable>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement { parts: Vec::new() }
    }

    pub fn push<V>(&mut self, variable: V)
        where V: Into<Variable>
    {
        self.parts.push(variable.into());
    }

    pub fn join<A>(self, separator: A) -> Statement
        where A: Into<Variable> + Clone
    {
        let mut it = self.parts.into_iter();

        let part: Variable = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<Variable> = Vec::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep.into());
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

impl<'a, T> From<&'a T> for Statement
    where T: Into<Statement> + Clone
{
    fn from(value: &'a T) -> Statement {
        value.clone().into()
    }
}

impl From<String> for Statement {
    fn from(value: String) -> Statement {
        Statement { parts: vec![Variable::Literal(value)] }
    }
}

impl From<Statement> for ElementSpec {
    fn from(value: Statement) -> ElementSpec {
        ElementSpec::Statement(value)
    }
}
