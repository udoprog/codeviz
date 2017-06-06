use common::ElementFormat;
use errors::*;
use super::elements::Elements;
use super::statement::Statement;
use super::variable_format::VariableFormat;

#[derive(Debug, Clone)]
pub enum ElementSpec<Var>
    where Var: VariableFormat
{
    Statement(Statement<Var>),
    Literal(String),
    Elements(Vec<ElementSpec<Var>>),
    Nested(Box<ElementSpec<Var>>),
    Spacing,
}

impl<Var> ElementSpec<Var>
    where Var: VariableFormat
{
    pub fn format<E>(&self, out: &mut E) -> Result<()>
        where E: ElementFormat
    {
        match *self {
            ElementSpec::Statement(ref statement) => {
                out.new_line_unless_empty()?;
                statement.format(out)?;
            }
            ElementSpec::Literal(ref line) => {
                out.new_line_unless_empty()?;
                out.write_str(line)?;
            }
            ElementSpec::Elements(ref elements) => {
                for element in elements {
                    element.format(out)?;
                }
            }
            ElementSpec::Nested(ref element) => {
                out.new_line_unless_empty()?;

                out.indent();
                element.format(out)?;
                out.unindent();
            }
            ElementSpec::Spacing => {
                out.new_line_unless_empty()?;
                out.new_line()?;
            }
        }

        Ok(())
    }
}

impl<'a, T, Var> From<&'a T> for ElementSpec<Var>
    where T: Into<ElementSpec<Var>> + Clone,
          Var: VariableFormat
{
    fn from(value: &'a T) -> ElementSpec<Var> {
        value.clone().into()
    }
}

impl<'a, Var> From<&'a str> for ElementSpec<Var>
    where Var: VariableFormat
{
    fn from(value: &'a str) -> ElementSpec<Var> {
        ElementSpec::Literal(value.to_owned())
    }
}

impl<Var> From<Elements<Var>> for ElementSpec<Var>
    where Var: VariableFormat
{
    fn from(value: Elements<Var>) -> ElementSpec<Var> {
        ElementSpec::Elements(value.elements)
    }
}

impl<Var> From<Vec<String>> for ElementSpec<Var>
    where Var: VariableFormat
{
    fn from(value: Vec<String>) -> ElementSpec<Var> {
        ElementSpec::Elements(value.into_iter().map(ElementSpec::Literal).collect())
    }
}

impl<Var> ToString for ElementSpec<Var>
    where Var: VariableFormat
{
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut ::common::ElementFormatter::new(&mut s)).unwrap();
        s
    }
}
