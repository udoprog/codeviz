use common::Element;
use super::elements::Elements;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct StructSpec {
    pub name: String,
    pub attributes: Elements,
    pub elements: Elements,
}

impl StructSpec {
    pub fn new(name: &str) -> StructSpec {
        StructSpec {
            name: name.to_owned(),
            attributes: Elements::new(),
            elements: Elements::new(),
        }
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
