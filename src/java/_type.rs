use errors::*;
use common::ElementFormat;
use super::variable::Variable;
use super::extra::Extra;

/// Complete types, including generic arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ClassType {
    pub package: String,
    pub name: String,
    pub arguments: Vec<Type>,
}

impl ClassType {
    pub fn new(package: &str, name: &str, arguments: Vec<Type>) -> ClassType {
        ClassType {
            package: package.to_owned(),
            name: name.to_owned(),
            arguments: arguments,
        }
    }

    pub fn with_arguments<A>(&self, arguments: Vec<A>) -> ClassType
        where A: Into<Type>
    {
        let arguments = arguments.into_iter().map(Into::into).collect();
        ClassType::new(&self.package, &self.name, arguments)
    }

    pub fn extend(&self, part: &str) -> ClassType {
        ClassType::new(&self.package,
                       &format!("{}.{}", self.name, part),
                       self.arguments.clone())
    }

    pub fn to_raw(&self) -> ClassType {
        ClassType::new(&self.package, &self.name, vec![])
    }

    pub fn format(&self, out: &mut ElementFormat, level: usize, extra: &mut Extra) -> Result<()> {
        // use fully qualified name if locals are occupied.
        if extra.absolute_import(&self.name, self) {
            write!(out, "{}.", self.package)?;
        }

        out.write_str(&self.name)?;

        if !self.arguments.is_empty() {
            let mut it = self.arguments.iter().peekable();

            let level = level + 1;

            out.write_char('<')?;

            while let Some(g) = it.next() {
                g.format(out, level, extra)?;

                if it.peek().is_some() {
                    out.write_str(", ")?;
                }
            }

            out.write_char('>')?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Local {
    pub name: String,
}

impl Local {
    pub fn new(name: &str) -> Local {
        Local { name: name.to_owned() }
    }

    pub fn format(&self, out: &mut ElementFormat) -> Result<()> {
        out.write_str(&self.name)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct PrimitiveType<'a> {
    pub primitive: &'a str,
    pub boxed: &'a str,
}

impl<'a> PrimitiveType<'a> {
    pub fn format(&self, out: &mut ElementFormat, level: usize) -> Result<()> {
        if level <= 0 {
            out.write_str(&self.primitive)?;
        } else {
            out.write_str(&self.boxed)?;
        }

        Ok(())
    }

    pub fn as_boxed(&self) -> ClassType {
        ClassType::new("java.lang", &self.boxed, vec![])
    }
}

/// Raw (importable) types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Type {
    Primitive(PrimitiveType<'static>),
    Class(ClassType),
    Local(Local),
}

impl Type {
    pub fn class(package: &str, name: &str) -> ClassType {
        ClassType::new(package, name, vec![])
    }

    pub fn local(name: &str) -> Local {
        Local::new(name)
    }

    pub fn format(&self, out: &mut ElementFormat, level: usize, extra: &mut Extra) -> Result<()> {
        match *self {
            Type::Primitive(ref primitive) => primitive.format(out, level),
            Type::Class(ref class) => class.format(out, level, extra),
            Type::Local(ref local) => local.format(out),
        }
    }
}

impl<'a, T> From<&'a T> for ClassType
    where T: Into<ClassType> + Clone
{
    fn from(value: &'a T) -> ClassType {
        value.clone().into()
    }
}

impl<'a, A> From<&'a A> for Type
    where A: Into<Type> + Clone
{
    fn from(value: &'a A) -> Type {
        value.clone().into()
    }
}

impl From<Type> for Variable {
    fn from(value: Type) -> Variable {
        Variable::Type(value)
    }
}

/// Implementation for ClassType to Type conversion.
impl From<ClassType> for Type {
    fn from(value: ClassType) -> Type {
        Type::Class(value)
    }
}

/// Implementation for PrimitiveType to Type conversion.
impl From<PrimitiveType<'static>> for Type {
    fn from(value: PrimitiveType<'static>) -> Type {
        Type::Primitive(value)
    }
}

impl From<Local> for Type {
    fn from(value: Local) -> Type {
        Type::Local(value)
    }
}

/// Primitive constants

pub const SHORT: PrimitiveType<'static> = PrimitiveType {
    primitive: "short",
    boxed: "Short",
};

pub const INTEGER: PrimitiveType<'static> = PrimitiveType {
    primitive: "int",
    boxed: "Integer",
};

pub const LONG: PrimitiveType<'static> = PrimitiveType {
    primitive: "long",
    boxed: "Long",
};

pub const FLOAT: PrimitiveType<'static> = PrimitiveType {
    primitive: "float",
    boxed: "Float",
};

pub const DOUBLE: PrimitiveType<'static> = PrimitiveType {
    primitive: "double",
    boxed: "Double",
};

pub const CHAR: PrimitiveType<'static> = PrimitiveType {
    primitive: "char",
    boxed: "Character",
};

pub const BOOLEAN: PrimitiveType<'static> = PrimitiveType {
    primitive: "boolean",
    boxed: "Boolean",
};

pub const BYTE: PrimitiveType<'static> = PrimitiveType {
    primitive: "byte",
    boxed: "Byte",
};

pub const VOID: PrimitiveType<'static> = PrimitiveType {
    primitive: "void",
    boxed: "Void",
};
