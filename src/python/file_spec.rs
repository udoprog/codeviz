use common::ElementFormatter;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::name::ImportedName;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub elements: Elements,
}

impl FileSpec {
    pub fn new() -> FileSpec {
        FileSpec { elements: Elements::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }

    pub fn format(&self) -> String {
        let mut s = String::new();

        let mut imports = BTreeSet::new();

        self.elements.imports(&mut imports);

        let modules: BTreeSet<(String, Option<String>)> =
            imports.into_iter().map(|imported| (imported.module, imported.alias)).collect();

        if !modules.is_empty() {
            for (module, alias) in modules {
                s.push_str("import ");
                s.push_str(&module);

                if let Some(ref alias) = alias {
                    s.push_str(" as ");
                    s.push_str(alias);
                }

                s.push('\n');
            }
        }

        let elements: ElementSpec = self.elements.clone().join(ElementSpec::Spacing).into();
        elements.format("", "  ", &mut ElementFormatter::new(&mut s)).unwrap();
        s
    }
}

impl ImportReceiver for BTreeSet<ImportedName> {
    fn receive(&mut self, name: &ImportedName) {
        self.insert(name.clone());
    }
}
