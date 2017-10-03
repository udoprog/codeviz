use super::*;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub name: String,
    pub constructors: Elements,
    pub elements: Elements,
    pub export: bool,
}

impl ClassSpec {
    pub fn new(name: &str) -> ClassSpec {
        ClassSpec {
            name: name.to_owned(),
            constructors: Elements::new(),
            elements: Elements::new(),
            export: false,
        }
    }

    pub fn export(&mut self) {
        self.export = true;
    }

    pub fn push_constructor<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.constructors.push(element);
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }
}

impl From<ClassSpec> for Element {
    fn from(value: ClassSpec) -> Element {
        let mut open = Statement::new();

        if value.export {
            open.push("export ");
        }

        open.push("class ");
        open.push(value.name);
        open.push(" {");

        let mut body = Elements::new();

        if !value.constructors.is_empty() {
            body.push(value.constructors.join(Spacing));
        }

        if !value.elements.is_empty() {
            body.push(value.elements.join(Spacing));
        }

        let mut out = Elements::new();
        out.push(open);
        out.push_nested(body.join(Spacing));
        out.push("}");

        out.into()
    }
}
