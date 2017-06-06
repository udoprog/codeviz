use super::_type::ClassType;
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::common::join_statements;
use super::element::*;
use super::elements::Elements;
use super::modifier::Modifiers;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    pub modifiers: Modifiers,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub throws: Vec<ClassType>,
    pub elements: Elements,
}

impl ConstructorSpec {
    pub fn new(modifiers: Modifiers) -> ConstructorSpec {
        ConstructorSpec {
            modifiers: modifiers,
            annotations: Vec::new(),
            arguments: Vec::new(),
            throws: Vec::new(),
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

    pub fn throws<T>(&mut self, throws: T)
        where T: Into<ClassType>
    {
        self.throws.push(throws.into())
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<Element>
    {
        self.elements.push(element);
    }

    pub fn as_element(self, enclosing: &str) -> Element {
        let mut elements = Elements::new();

        let mut open = Statement::new();

        for a in self.annotations {
            elements.push(a);
        }

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        open.push(enclosing);

        if self.arguments.is_empty() {
            open.push("()");
        } else {
            open.push("(");
            open.push(Nested(Box::new(Push(join_statements(self.arguments, ", ")))));
            open.push(Variable::Element(Push(Variable::Literal(String::from(")")).into())));
        }

        if !self.throws.is_empty() {
            open.push(" throws ");

            let mut arguments = Statement::new();

            for throw in self.throws {
                arguments.push(throw);
            }

            open.push(arguments.join(", "));
        }

        open.push(" {");

        elements.push(open);
        elements.push_nested(self.elements);
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
