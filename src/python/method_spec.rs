use super::decorator_spec::DecoratorSpec;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub name: String,
    pub decorators: Vec<DecoratorSpec>,
    pub arguments: Vec<Statement>,
    pub elements: Elements,
}

impl MethodSpec {
    pub fn new(name: &str) -> MethodSpec {
        MethodSpec {
            name: name.to_owned(),
            decorators: Vec::new(),
            arguments: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_decorator<D>(&mut self, decorator: D)
        where D: Into<DecoratorSpec>
    {
        self.decorators.push(decorator.into());
    }

    pub fn push_argument<S>(&mut self, argument: S)
        where S: Into<Statement>
    {
        self.arguments.push(argument.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }
}
