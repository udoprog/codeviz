use super::_type::{Type, ClassType};
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct AnnotationSpec {
    pub ty: Type,
    pub arguments: Vec<Statement>,
}

impl AnnotationSpec {
    pub fn new<I>(ty: I) -> AnnotationSpec
        where I: Into<Type>
    {
        AnnotationSpec {
            ty: ty.into(),
            arguments: Vec::new(),
        }
    }

    pub fn push_argument<S>(&mut self, statement: S)
        where S: Into<Statement>
    {
        self.arguments.push(statement.into());
    }
}

impl<'a, T> From<&'a T> for AnnotationSpec
    where T: Into<AnnotationSpec> + Clone
{
    fn from(value: &'a T) -> AnnotationSpec {
        value.clone().into()
    }
}

impl From<ClassType> for AnnotationSpec {
    fn from(value: ClassType) -> AnnotationSpec {
        AnnotationSpec::new(value)
    }
}
