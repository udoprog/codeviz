use errors::*;
use common::ElementFormat;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum Element {
    Statement(Statement),
    Literal(String),
    Elements(Vec<Element>),
    Nested(Box<Element>),
    Spacing,
}

impl Element {
    pub fn format<E>(&self, out: &mut E) -> Result<()>
        where E: ElementFormat
    {
        match *self {
            Element::Statement(ref statement) => {
                out.new_line_unless_empty()?;
                statement.format(out)?;
            }
            Element::Literal(ref line) => {
                out.new_line_unless_empty()?;
                out.write_str(line)?;
            }
            Element::Elements(ref elements) => {
                for element in elements {
                    element.format(out)?;
                }
            }
            Element::Nested(ref element) => {
                out.new_line_unless_empty()?;

                out.indent();
                element.format(out)?;
                out.unindent();
            }
            Element::Spacing => {
                out.new_line_unless_empty()?;
                out.new_line()?;
            }
        };

        Ok(())
    }
}

impl<'a, T> From<&'a T> for Element
    where T: Into<Element> + Clone
{
    fn from(value: &'a T) -> Element {
        value.clone().into()
    }
}

impl<'a> From<&'a str> for Element {
    fn from(value: &'a str) -> Element {
        Element::Literal(value.to_owned())
    }
}

impl From<Statement> for Element {
    fn from(value: Statement) -> Element {
        Element::Statement(value)
    }
}

impl From<Vec<String>> for Element {
    fn from(value: Vec<String>) -> Element {
        Element::Elements(value.into_iter().map(Element::Literal).collect())
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut ::common::ElementFormatter::new(&mut s)).unwrap();
        s
    }
}
