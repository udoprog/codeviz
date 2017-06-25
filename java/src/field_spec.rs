use super::*;

#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub modifiers: Modifiers,
    pub ty: Type,
    pub name: String,
    pub initialize: Option<Statement>,
}

impl FieldSpec {
    pub fn new<I>(modifiers: Modifiers, ty: I, name: &str) -> FieldSpec
        where I: Into<Type>
    {
        FieldSpec {
            modifiers: modifiers,
            ty: ty.into(),
            name: name.to_owned(),
            initialize: None,
        }
    }

    pub fn initialize<S>(&mut self, initialize: S)
        where S: Into<Statement>
    {
        self.initialize = Some(initialize.into());
    }
}

impl<'a, T> From<&'a T> for FieldSpec
    where T: Into<FieldSpec> + Clone
{
    fn from(value: &'a T) -> FieldSpec {
        value.clone().into()
    }
}

impl From<FieldSpec> for Variable {
    fn from(value: FieldSpec) -> Variable {
        Variable::Literal(value.name)
    }
}

impl From<FieldSpec> for Statement {
    fn from(value: FieldSpec) -> Statement {
        let mut s = Statement::new();

        if !value.modifiers.is_empty() {
            s.push(value.modifiers);
            s.push(" ");
        }

        s.push(value.ty);
        s.push(" ");
        s.push(value.name);

        if let Some(initialize) = value.initialize {
            s.push(" = ");
            s.push(initialize);
        }

        s
    }
}
