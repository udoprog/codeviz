use common::Element;
use super::elements::Elements;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct StructSpec {
    pub name: String,
    pub attributes: Elements,
    pub elements: Elements,
    pub public: bool,
}

impl StructSpec {
    pub fn new(name: &str) -> StructSpec {
        StructSpec {
            name: name.to_owned(),
            attributes: Elements::new(),
            elements: Elements::new(),
            public: false,
        }
    }

    pub fn public(&mut self) {
        self.public = true;
    }

    pub fn push_attribute<D>(&mut self, attribute: D)
        where D: Into<Element<Variable>>
    {
        self.attributes.push(attribute.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<Element<Variable>>
    {
        self.elements.push(element);
    }
}

impl From<StructSpec> for Element<Variable> {
    fn from(value: StructSpec) -> Element<Variable> {
        let mut out = Elements::new();

        out.push(value.attributes);

        let mut decl = Statement::new();

        if value.public {
            decl.push("pub ");
        }

        decl.push("struct ");
        decl.push(value.name);
        decl.push(" {");

        out.push(decl);

        if !value.elements.is_empty() {
            out.push_nested(value.elements.join(Element::Spacing));
        }

        out.push("}");

        out.into()
    }
}
