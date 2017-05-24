use super::_type::{Type, ClassType};
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::field_spec::FieldSpec;
use super::modifier::Modifiers;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum Variable {
    Literal(String),
    Type(Type),
    String(String),
    Statement(Statement),
    Spacing,
}

impl<'a, T> From<&'a T> for Variable
    where T: Into<Variable> + Clone
{
    fn from(value: &'a T) -> Variable {
        value.clone().into()
    }
}

impl<'a> From<&'a str> for Variable {
    fn from(value: &'a str) -> Variable {
        Variable::Literal(value.to_owned())
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Variable {
        Variable::Literal(value)
    }
}

impl From<Statement> for Variable {
    fn from(value: Statement) -> Variable {
        Variable::Statement(value)
    }
}

impl From<FieldSpec> for Variable {
    fn from(value: FieldSpec) -> Variable {
        Variable::Literal(value.name)
    }
}

impl From<ArgumentSpec> for Variable {
    fn from(value: ArgumentSpec) -> Variable {
        Variable::Literal(value.name)
    }
}

impl From<Modifiers> for Variable {
    fn from(value: Modifiers) -> Variable {
        Variable::Literal(value.format())
    }
}

impl From<Type> for Variable {
    fn from(value: Type) -> Variable {
        Variable::Type(value)
    }
}

impl From<ClassType> for Variable {
    fn from(value: ClassType) -> Variable {
        Variable::Type(value.into())
    }
}

impl From<AnnotationSpec> for Variable {
    fn from(value: AnnotationSpec) -> Variable {
        Variable::Statement(value.into())
    }
}
