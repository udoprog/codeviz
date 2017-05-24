use super::_type::Type;
use super::modifier::Modifiers;
use super::statement::Statement;

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
