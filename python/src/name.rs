use codeviz_common::ElementFormat;
use super::*;

#[derive(Debug, Clone)]
pub enum Name {
    Imported(ImportedName),
    BuiltIn(BuiltInName),
    Local(LocalName),
}

impl Name {
    pub fn imported(module: &str, name: &str) -> ImportedName {
        ImportedName {
            module: module.to_owned(),
            name: name.to_owned(),
            alias: None,
        }
    }

    pub fn imported_alias(module: &str, name: &str, alias: &str) -> ImportedName {
        ImportedName {
            module: module.to_owned(),
            name: name.to_owned(),
            alias: Some(alias.to_owned()),
        }
    }

    pub fn built_in(name: &str) -> BuiltInName {
        BuiltInName { name: name.to_owned() }
    }

    pub fn local(name: &str) -> LocalName {
        LocalName { name: name.to_owned() }
    }

    pub fn format<E>(&self, out: &mut E) -> Result<()>
    where
        E: ElementFormat,
    {
        match *self {
            Name::Imported(ref imported) => {
                if let Some(ref alias) = imported.alias {
                    write!(out, "{}.{}", alias, imported.name.clone())?;
                } else {
                    write!(out, "{}.{}", imported.module, imported.name.clone())?;
                }
            }
            Name::BuiltIn(ref built_in) => out.write_str(&built_in.name)?,
            Name::Local(ref local) => out.write_str(&local.name)?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImportedName {
    pub module: String,
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuiltInName {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LocalName {
    pub name: String,
}

impl<'a, T> From<&'a T> for Name
where
    T: Into<Name> + Clone,
{
    fn from(value: &'a T) -> Name {
        value.clone().into()
    }
}

impl From<ImportedName> for Name {
    fn from(value: ImportedName) -> Name {
        Name::Imported(value)
    }
}

impl From<BuiltInName> for Name {
    fn from(value: BuiltInName) -> Name {
        Name::BuiltIn(value)
    }
}

impl From<LocalName> for Name {
    fn from(value: LocalName) -> Name {
        Name::Local(value)
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
