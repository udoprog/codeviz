use super::element_spec::ElementSpec;
use super::elements::Elements;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub name: String,
    pub constructors: Elements,
    pub elements: Elements,
}

impl ClassSpec {
    pub fn new(name: &str) -> ClassSpec {
        ClassSpec {
            name: name.to_owned(),
            constructors: Elements::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_constructor<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.constructors.push(element);
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }
}
