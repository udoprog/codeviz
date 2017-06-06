use common::ElementFormatter;
use common::ElementSpec;
use errors::*;
use std::collections::BTreeSet;
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::name::ImportedName;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub elements: Elements,
}

impl FileSpec {
    pub fn new() -> FileSpec {
        FileSpec { elements: Elements::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec<Variable>>
    {
        self.elements.push(element);
    }

    fn imports(&self) -> Option<Elements> {
        let mut imports = BTreeSet::new();

        self.elements.imports(&mut imports);

        let modules: BTreeSet<(String, Option<String>)> =
            imports.into_iter().map(|imported| (imported.module, imported.alias)).collect();

        if modules.is_empty() {
            return None;
        }

        let mut elements = Elements::new();

        for (module, alias) in modules {
            let mut s = Statement::new();

            s.push("import ");
            s.push(&module);

            if let Some(ref alias) = alias {
                s.push(" as ");
                s.push(alias);
            }

            elements.push(s);
        }

        Some(elements)
    }

    pub fn format<W>(&self, out: &mut W) -> Result<()>
        where W: ::std::fmt::Write
    {
        let mut elements = Elements::new();

        if let Some(imports) = self.imports() {
            elements.push(imports);
        }

        elements.push(self.elements.clone().join(ElementSpec::Spacing));

        let elements: ElementSpec<Variable> = elements.clone().join(ElementSpec::Spacing).into();

        elements.format(&mut ElementFormatter::new(out))?;
        out.write_char('\n')?;

        Ok(())
    }
}

impl ImportReceiver for BTreeSet<ImportedName> {
    fn receive(&mut self, name: &ImportedName) {
        self.insert(name.clone());
    }
}

impl ToString for FileSpec {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut s).unwrap();
        s
    }
}
