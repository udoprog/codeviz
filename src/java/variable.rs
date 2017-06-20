use common::ElementFormat;
use common::VariableFormat;
use errors::*;
use super::_type::Type;
use super::element::Element;
use super::statement::Statement;
use super::extra::Extra;

#[derive(Debug, Clone)]
pub enum Variable {
    Literal(String),
    Type(Type),
    String(String),
    Statement(Statement),
    Element(Element),
    Spacing,
}

impl VariableFormat for Variable {
    type Extra = Extra;

    fn format<E>(&self, out: &mut E, depth: usize, extra: &mut Extra) -> Result<()>
        where E: ElementFormat
    {
        match *self {
            Variable::Type(ref ty) => ty.format(out, depth, extra)?,
            Variable::String(ref string) => java_quote_string(out, string)?,
            Variable::Statement(ref stmt) => stmt.format(out, depth, extra)?,
            Variable::Literal(ref content) => out.write_str(content)?,
            Variable::Element(ref element) => element.format(out, extra)?,
            Variable::Spacing => out.new_line()?,
        };

        Ok(())
    }
}

impl<'a, T> From<&'a T> for Variable
    where T: Into<Variable> + Clone
{
    fn from(value: &'a T) -> Variable {
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

impl From<Element> for Variable {
    fn from(value: Element) -> Variable {
        Variable::Element(value)
    }
}

fn java_quote_string(out: &mut ElementFormat, input: &str) -> Result<()> {
    out.write_char('"')?;

    for c in input.chars() {
        match c {
            '\t' => out.write_str("\\t")?,
            '\u{0007}' => out.write_str("\\b")?,
            '\n' => out.write_str("\\n")?,
            '\r' => out.write_str("\\r")?,
            '\u{0014}' => out.write_str("\\f")?,
            '\'' => out.write_str("\\'")?,
            '"' => out.write_str("\\\"")?,
            '\\' => out.write_str("\\\\")?,
            c => out.write_char(c)?,
        }
    }

    out.write_char('"')?;

    Ok(())
}
