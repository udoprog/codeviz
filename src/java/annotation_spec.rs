use super::_type::{Type, ClassType};
use super::element::*;
use super::elements::Elements;
use super::statement::Statement;
use super::variable::Variable;

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

impl From<AnnotationSpec> for Element {
    fn from(value: AnnotationSpec) -> Element {
        let mut elements = Elements::new();

        let mut annotation = Statement::new();
        annotation.push("@");
        annotation.push(value.ty);

        if !value.arguments.is_empty() {
            let mut open = Statement::new();

            let arguments: Statement = value.arguments.into();

            open.push(annotation);
            open.push("(");
            open.push(arguments.join(", "));
            open.push(")");

            elements.push(open);
        } else {
            elements.push(annotation);
        }

        elements.into()
    }
}

impl From<AnnotationSpec> for Statement {
    fn from(value: AnnotationSpec) -> Statement {
        Statement { parts: vec![Variable::Element(value.into())] }
    }
}
