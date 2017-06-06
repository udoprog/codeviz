use errors::*;
use common::VariableFormat;
use common::ElementFormat;
use super::name::{Name, ImportedName, BuiltInName, LocalName};
use super::statement::Statement;

/// Variables that are part of statements.
#[derive(Debug, Clone)]
pub enum Variable {
    /// String that will be literally appended.
    Literal(String),
    /// String that will be quoted and appended.
    String(String),
    /// Another statement that will be appended.
    Statement(Statement),
    /// A name that will be appended.
    Name(Name),
}

impl VariableFormat for Variable {
    fn format<E>(&self, out: &mut E, depth: usize) -> Result<()>
        where E: ElementFormat
    {
        match *self {
            Variable::String(ref string) => {
                quote_string(out, string)?;
            }
            Variable::Statement(ref stmt) => {
                stmt.format(out, depth)?;
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

impl From<Statement> for Variable {
    fn from(value: Statement) -> Variable {
        Variable::Statement(value)
    }
}

impl From<Name> for Variable {
    fn from(value: Name) -> Variable {
        Variable::Name(value)
    }
}

impl From<ImportedName> for Variable {
    fn from(value: ImportedName) -> Variable {
        Variable::Name(value.into())
    }
}

impl From<BuiltInName> for Variable {
    fn from(value: BuiltInName) -> Variable {
        Variable::Name(value.into())
    }
}

impl From<LocalName> for Variable {
    fn from(value: LocalName) -> Variable {
        Variable::Name(value.into())
    }
}

/// Quote a string to make it suitable as a literal JavaScript string.
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
