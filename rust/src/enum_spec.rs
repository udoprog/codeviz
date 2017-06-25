use super::*;

#[derive(Debug, Clone)]
pub struct EnumSpec {
    pub name: String,
    pub attributes: Elements,
    pub elements: Elements,
    pub public: bool,
}

impl EnumSpec {
    pub fn new(name: &str) -> EnumSpec {
        EnumSpec {
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
        where D: Into<Element>
    {
        self.attributes.push(attribute.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<Element>
    {
        self.elements.push(element);
    }
}

impl From<EnumSpec> for Element {
    fn from(value: EnumSpec) -> Element {
        let mut out = Elements::new();

        out.push(value.attributes);

        let mut decl = Statement::new();

        if value.public {
            decl.push("pub ");
        }

        decl.push("enum ");
        decl.push(value.name);
        decl.push(" {");

        out.push(decl);

        if !value.elements.is_empty() {
            out.push_nested(value.elements.join(Spacing));
        }

        out.push("}");

        out.into()
    }
}
