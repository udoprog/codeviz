use common::Element;
use super::elements::Elements;
use super::decorator_spec::DecoratorSpec;
use super::statement::Statement;
use super::variable::Variable;

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
        where E: Into<Element<Variable>>
    {
        self.elements.push(element);
    }
}

impl From<MethodSpec> for Element<Variable> {
    fn from(value: MethodSpec) -> Element<Variable> {
        let mut out: Vec<Element<Variable>> = Vec::new();

        for decorator in value.decorators {
            out.push(decorator.into());
        }

        let mut decl = Statement::new();
        decl.push("def ");
        decl.push(value.name);
        decl.push("(");

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        decl.push(arguments.join(", "));
        decl.push("):");

        out.push(decl.into());

        if value.elements.is_empty() {
            out.push(Element::Nested(Box::new("pass".into())));
        } else {
            out.push(Element::Nested(Box::new(value.elements.into())));
        }

        Element::Elements(out)
    }
}
