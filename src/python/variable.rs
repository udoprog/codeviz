use super::statement::Statement;
use super::name::{Name, ImportedName, BuiltInName, LocalName};

/// Variables that are part of statements.
#[derive(Debug, Clone)]
pub enum Variable {
    /// String that will be literally appended.
    Literal(String),
    /// String that will be quoted and appended.
    String(String),
    /// Another statement that will be appended.
    Statement(Statement),
    /// A name that will be appended.
    Name(Name),
}

impl<'a, A> From<&'a A> for Variable
    where A: Into<Variable> + Clone
{
    fn from(value: &'a A) -> Variable {
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

impl From<Name> for Variable {
    fn from(value: Name) -> Variable {
        Variable::Name(value)
    }
}

impl From<ImportedName> for Variable {
    fn from(value: ImportedName) -> Variable {
        Variable::Name(value.into())
    }
}

impl From<BuiltInName> for Variable {
    fn from(value: BuiltInName) -> Variable {
        Variable::Name(value.into())
    }
}

impl From<LocalName> for Variable {
    fn from(value: LocalName) -> Variable {
        Variable::Name(value.into())
    }
}
