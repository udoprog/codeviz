use errors::*;
use common::ElementFormat;
use super::name::Name;
use common::Statement;
use common::VariableFormat;

/// Variables that are part of statements.
#[derive(Debug, Clone)]
pub enum Variable {
    /// String that will be literally appended.
    Literal(String),
    /// String that will be quoted and appended.
    String(String),
    /// Another statement that will be appended.
    Statement(Statement<Variable>),
    /// A name that will be appended.
    Name(Name),
}

impl VariableFormat for Variable {
    fn format<E>(&self, out: &mut E) -> Result<()>
        where E: ElementFormat
    {
        match *self {
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

        Ok(())
    }
}

impl<'a, A> From<&'a A> for Variable
    where A: Into<Variable> + Clone
{
    fn from(value: &'a A) -> Variable {
        value.clone().into()
    }
}

impl<'a> From<&'a str> for Variable {
    fn from(value: &'a str) -> Variable {
        Variable::Literal(value.to_owned())
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Variable {
        Variable::Literal(value)
    }
}

impl From<Statement<Variable>> for Variable {
    fn from(value: Statement<Variable>) -> Variable {
        Variable::Statement(value)
    }
}

impl From<Variable> for Statement<Variable> {
    fn from(value: Variable) -> Statement<Variable> {
        Statement { parts: vec![value] }
    }
}

/// Quote a string to make it suitable as a literal Python string.
fn quote_string<E>(out: &mut E, input: &str) -> Result<()>
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
