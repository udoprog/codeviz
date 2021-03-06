use super::*;

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    pub arguments: Vec<Statement>,
    pub elements: Elements,
}

impl ConstructorSpec {
    pub fn new() -> ConstructorSpec {
        ConstructorSpec {
            arguments: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_argument<S>(&mut self, argument: S)
    where
        S: Into<Statement>,
    {
        self.arguments.push(argument.into());
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }
}

impl From<ConstructorSpec> for Element {
    fn from(value: ConstructorSpec) -> Element {
        let mut open = Statement::new();

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        open.push("constructor(");
        open.push(arguments.join(", "));
        open.push(")");

        open.push(" {");

        let mut out = Elements::new();

        out.push(open);
        out.push_nested(value.elements.join(Spacing));
        out.push("}");

        out.into()
    }
}
