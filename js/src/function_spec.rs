use super::*;

#[derive(Debug, Clone)]
pub struct FunctionSpec {
    pub name: String,
    pub arguments: Vec<Statement>,
    pub elements: Elements,
}

impl FunctionSpec {
    pub fn new(name: &str) -> FunctionSpec {
        FunctionSpec {
            name: name.to_owned(),
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

impl From<FunctionSpec> for Element {
    fn from(value: FunctionSpec) -> Element {
        let mut open = Statement::new();
        open.push("function ");
        open.push(value.name);
        open.push("(");

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        open.push(arguments.join(", "));
        open.push(") {");

        let mut out = Elements::new();
        out.push(open);
        out.push_nested(value.elements.join(Spacing));
        out.push("}");

        out.into()
    }
}
