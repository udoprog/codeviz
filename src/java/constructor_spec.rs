use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::modifier::Modifiers;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    pub modifiers: Modifiers,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub elements: Elements,
}

impl ConstructorSpec {
    pub fn new(modifiers: Modifiers) -> ConstructorSpec {
        ConstructorSpec {
            modifiers: modifiers,
            annotations: Vec::new(),
            arguments: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_annotation<A>(&mut self, annotation: A)
        where A: Into<AnnotationSpec>
    {
        self.annotations.push(annotation.into());
    }

    pub fn push_argument<A>(&mut self, argument: A)
        where A: Into<ArgumentSpec>
    {
        self.arguments.push(argument.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }

    pub fn as_element_spec(&self, enclosing: &str) -> ElementSpec {
        let mut elements = Elements::new();

        let mut open = Statement::new();

        for a in &self.annotations {
            elements.push(a);
        }

        if !self.modifiers.is_empty() {
            open.push(&self.modifiers);
            open.push(" ");
        }

        open.push(enclosing);
        open.push("(");
        open.push(Statement::join_statements(&self.arguments, ", "));
        open.push(") {");

        elements.push(open);
        elements.push_nested(&self.elements);
        elements.push("}");

        elements.into()
    }
}

impl<'a, T> From<&'a T> for ConstructorSpec
    where T: Into<ConstructorSpec> + Clone
{
    fn from(value: &'a T) -> ConstructorSpec {
        value.clone().into()
    }
}
