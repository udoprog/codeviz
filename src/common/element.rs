use common::ElementFormat;
use errors::*;
use super::elements::Elements;
use super::statement::Statement;
use super::variable_format::VariableFormat;

#[derive(Debug, Clone)]
pub enum Element<Var>
    where Var: VariableFormat
{
    Statement(Statement<Var>),
    Literal(String),
    Elements(Vec<Element<Var>>),
    Nested(Box<Element<Var>>),
    Spacing,
}

impl<Var> Element<Var>
    where Var: VariableFormat
{
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
        }

        Ok(())
    }
}

impl<'a, T, Var> From<&'a T> for Element<Var>
    where T: Into<Element<Var>> + Clone,
          Var: VariableFormat
{
    fn from(value: &'a T) -> Element<Var> {
        value.clone().into()
    }
}

impl<'a, Var> From<&'a str> for Element<Var>
    where Var: VariableFormat
{
    fn from(value: &'a str) -> Element<Var> {
        Element::Literal(value.to_owned())
    }
}

impl<Var> From<Elements<Var>> for Element<Var>
    where Var: VariableFormat
{
    fn from(value: Elements<Var>) -> Element<Var> {
        Element::Elements(value.elements)
    }
}

impl<Var> From<Vec<String>> for Element<Var>
    where Var: VariableFormat
{
    fn from(value: Vec<String>) -> Element<Var> {
        Element::Elements(value.into_iter().map(Element::Literal).collect())
    }
}

impl<Var> ToString for Element<Var>
    where Var: VariableFormat
{
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut ::common::ElementFormatter::new(&mut s)).unwrap();
        s
    }
}
