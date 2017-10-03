use super::*;

#[derive(Debug, Clone)]
pub struct DecoratorSpec {
    pub name: Name,
    pub arguments: Vec<Statement>,
}

impl DecoratorSpec {
    pub fn new<N>(name: N) -> DecoratorSpec
    where
        N: Into<Name>,
    {
        DecoratorSpec {
            name: name.into(),
            arguments: Vec::new(),
        }
    }

    pub fn push_argument<S>(&mut self, statement: S)
    where
        S: Into<Statement>,
    {
        self.arguments.push(statement.into());
    }
}

impl<'a, T> From<&'a T> for DecoratorSpec
where
    T: Into<DecoratorSpec> + Clone,
{
    fn from(value: &'a T) -> DecoratorSpec {
        value.clone().into()
    }
}

impl From<BuiltInName> for DecoratorSpec {
    fn from(value: BuiltInName) -> DecoratorSpec {
        DecoratorSpec::new(value)
    }
}

impl From<ImportedName> for DecoratorSpec {
    fn from(value: ImportedName) -> DecoratorSpec {
        DecoratorSpec::new(value)
    }
}

impl From<DecoratorSpec> for Element {
    fn from(value: DecoratorSpec) -> Element {
        let mut decl = Statement::new();

        decl.push("@");
        decl.push(value.name);

        decl.into()
    }
}
