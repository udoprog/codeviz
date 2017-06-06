use super::decorator_spec::DecoratorSpec;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::name::Name;
use super::statement::Statement;

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
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }

    pub fn extends<N>(&mut self, name: N)
        where N: Into<Name>
    {
        self.extends.push(name.into());
    }
}

impl From<ClassSpec> for ElementSpec {
    fn from(value: ClassSpec) -> ElementSpec {
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
            out.push_nested(value.elements.join(ElementSpec::Spacing));
        }

        out.into()
    }
}
