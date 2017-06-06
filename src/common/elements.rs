use super::element_spec::ElementSpec;
use super::variable_format::VariableFormat;

#[derive(Debug, Clone)]
pub struct Elements<Var>
    where Var: VariableFormat
{
    pub elements: Vec<ElementSpec<Var>>,
}

impl<Var> Elements<Var>
    where Var: VariableFormat
{
    pub fn new() -> Elements<Var> {
        Elements { elements: Vec::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec<Var>>
    {
        self.elements.push(element.into());
    }

    pub fn push_nested<E>(&mut self, element: E)
        where E: Into<ElementSpec<Var>>
    {
        self.elements.push(ElementSpec::Nested(Box::new(element.into())));
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn join<S>(self, separator: S) -> Elements<Var>
        where S: Into<ElementSpec<Var>> + Clone
    {
        let mut it = self.elements.into_iter();

        let part = match it.next() {
            Some(part) => part,
            None => return Elements::new(),
        };

        let mut parts: Elements<Var> = Elements::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep);
            parts.push(part);
        }

        parts
    }
}
