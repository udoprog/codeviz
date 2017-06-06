use common::Element;
use super::decorator_spec::DecoratorSpec;
use super::elements::Elements;
use super::name::Name;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub name: String,
    pub decorators: Vec<DecoratorSpec>,
    pub elements: Elements,
    pub extends: Vec<Name>,
}

impl ClassSpec {
    pub fn new(name: &str) -> ClassSpec {
        ClassSpec {
            name: name.to_owned(),
            decorators: Vec::new(),
            elements: Elements::new(),
            extends: Vec::new(),
        }
    }

    pub fn push_decorator<D>(&mut self, decorator: D)
        where D: Into<DecoratorSpec>
    {
        self.decorators.push(decorator.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<Element<Variable>>
    {
        self.elements.push(element);
    }

    pub fn extends<N>(&mut self, name: N)
        where N: Into<Name>
    {
        self.extends.push(name.into());
    }
}

impl From<ClassSpec> for Element<Variable> {
    fn from(value: ClassSpec) -> Element<Variable> {
        let mut out = Elements::new();

        for decorator in value.decorators {
            out.push(decorator);
        }

        let mut decl = Statement::new();
        decl.push("class ");
        decl.push(value.name);

        if !value.extends.is_empty() {
            decl.push("(");

            let mut extends = Statement::new();

            for extend in value.extends {
                extends.push(extend);
            }

            decl.push(extends.join(", "));
            decl.push(")");
        }

        decl.push(":");

        out.push(decl);

        if value.elements.is_empty() {
            out.push_nested("pass");
        } else {
            out.push_nested(value.elements.join(Element::Spacing));
        }

        out.into()
    }
}
