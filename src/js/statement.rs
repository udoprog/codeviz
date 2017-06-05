use errors::*;
use common::ElementFormat;
use super::variable::Variable;

/// Quote a string to make it suitable as a literal Python string.
pub fn quote_string<E>(out: &mut E, input: &str) -> Result<()>
    where E: ElementFormat
{
    out.write_char('"')?;

    for c in input.chars() {
        match c {
            '\t' => out.write_str("\\t"),
            '\u{0007}' => out.write_str("\\b"),
            '\n' => out.write_str("\\n"),
            '\r' => out.write_str("\\r"),
            '\u{0014}' => out.write_str("\\f"),
            '\'' => out.write_str("\\'"),
            '"' => out.write_str("\\\""),
            '\\' => out.write_str("\\\\"),
            c => out.write_char(c),
        }?;
    }

    out.write_char('"')?;

    Ok(())
}

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
            match *part {
                Variable::String(ref string) => {
                    quote_string(out, string)?;
                }
                Variable::Statement(ref stmt) => {
                    stmt.format(out)?;
                }
                Variable::Literal(ref content) => {
                    out.write_str(content)?;
                }
                Variable::Name(ref name) => {
                    name.format(out)?;
                }
            }
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

impl From<Variable> for Statement {
    fn from(value: Variable) -> Statement {
        Statement { parts: vec![value] }
    }
}

impl From<String> for Statement {
    fn from(value: String) -> Statement {
        Statement { parts: vec![Variable::Literal(value)] }
    }
}
