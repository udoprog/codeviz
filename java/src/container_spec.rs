use super::*;

pub trait ContainerSpec {
    fn push<E>(&mut self, element: E)
    where
        E: Into<Element>;
}

impl ContainerSpec for InterfaceSpec {
    fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }
}

impl ContainerSpec for ClassSpec {
    fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }
}

impl ContainerSpec for EnumSpec {
    fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }
}
