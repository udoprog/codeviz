use super::element_spec::ElementSpec;

#[derive(Debug, Clone)]
pub struct Elements {
    pub elements: Vec<ElementSpec>,
}

impl Elements {
    pub fn new() -> Elements {
        Elements { elements: Vec::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element.into());
    }

    pub fn push_nested<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(ElementSpec::Nested(Box::new(element.into())));
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn join<S>(self, separator: S) -> Elements
        where S: Into<ElementSpec> + Clone
    {
        let mut it = self.elements.into_iter();

        let part = match it.next() {
            Some(part) => part,
            None => return Elements::new(),
        };

        let mut parts: Elements = Elements::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep);
            parts.push(part);
        }

        parts
    }
}
