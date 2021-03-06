use super::*;

#[derive(Debug, Clone)]
pub enum Element<Var>
where
    Var: VariableFormat,
{
    Push(Statement<Var>),
    Concat(Statement<Var>),
    Literal(String),
    Inner(Vec<Element<Var>>),
    Nested(Box<Element<Var>>),
    Spacing,
}

impl<Var> Element<Var>
where
    Var: VariableFormat,
{
    pub fn format<E>(&self, out: &mut E, extra: &mut Var::Extra) -> Result<()>
    where
        E: ElementFormat,
    {
        match *self {
            Element::Push(ref statement) => {
                out.new_line_unless_empty()?;
                statement.format(out, 0usize, extra)?;
            }
            Element::Concat(ref statement) => {
                statement.format(out, 0usize, extra)?;
            }
            Element::Literal(ref line) => {
                out.new_line_unless_empty()?;
                out.write_str(line)?;
            }
            Element::Inner(ref elements) => {
                for element in elements {
                    element.format(out, extra)?;
                }
            }
            Element::Nested(ref element) => {
                out.new_line_unless_empty()?;

                out.indent();
                element.format(out, extra)?;
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
where
    T: Into<Element<Var>> + Clone,
    Var: VariableFormat,
{
    fn from(value: &'a T) -> Element<Var> {
        value.clone().into()
    }
}

impl<'a, Var> From<&'a str> for Element<Var>
where
    Var: VariableFormat,
{
    fn from(value: &'a str) -> Element<Var> {
        Element::Literal(value.to_owned())
    }
}

impl<Var> From<Elements<Var>> for Element<Var>
where
    Var: VariableFormat,
{
    fn from(value: Elements<Var>) -> Element<Var> {
        Element::Inner(value.elements)
    }
}

impl<Var> From<Vec<String>> for Element<Var>
where
    Var: VariableFormat,
{
    fn from(value: Vec<String>) -> Element<Var> {
        Element::Inner(value.into_iter().map(Element::Literal).collect())
    }
}

impl<Var> ToString for Element<Var>
where
    Var: VariableFormat,
{
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut extra = Var::Extra::default();
        self.format(&mut ElementFormatter::new(&mut s), &mut extra)
            .unwrap();
        s
    }
}
