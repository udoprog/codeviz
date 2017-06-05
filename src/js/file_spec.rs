use common::ElementFormatter;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::name::ImportedName;
use super::statement::{self, Statement};

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

    fn module_to_path(&self, path: &str) -> String {
        let parts: Vec<&str> = path.split(".").collect();
        format!("{}.js", parts.join("/"))
    }

    fn imports(&self) -> Elements {
        let mut imports = BTreeSet::new();
        self.elements.imports(&mut imports);

        // specific imports from modules.
        let mut collected: BTreeMap<String, Vec<String>> = BTreeMap::new();
        // * imported modules, and their alias.
        let mut wildcard: BTreeSet<(String, String)> = BTreeSet::new();

        for import in imports {
            if let Some(alias) = import.alias {
                wildcard.insert((import.module, alias));
            } else {
                match collected.entry(import.module.clone()) {
                    btree_map::Entry::Vacant(entry) => {
                        entry.insert(vec![import.name.clone()]);
                    }
                    btree_map::Entry::Occupied(entry) => {
                        entry.into_mut().push(import.name.clone());
                    }
                }
            }
        }

        let mut out = Elements::new();

        for (module, names) in collected {
            let mut stmt = Statement::new();

            stmt.push("import ");
            stmt.push("{");
            stmt.push(names.join(", "));
            stmt.push("}");
            stmt.push(" from ");
            stmt.push(statement::quote_string(&self.module_to_path(&module)));
            stmt.push(";");

            out.push(stmt);
        }

        for (module, alias) in wildcard {
            let mut stmt = Statement::new();

            stmt.push("import * as ");
            stmt.push(alias);
            stmt.push(" from ");
            stmt.push(statement::quote_string(&self.module_to_path(&module)));
            stmt.push(";");

            out.push(stmt);
        }

        out
    }

    pub fn format(&self) -> String {
        let mut out = Elements::new();

        out.push(self.imports());
        out.push(self.elements.clone().join(ElementSpec::Spacing));

        let elements: ElementSpec = out.join(ElementSpec::Spacing).into();

        let mut s = String::new();
        elements.format("", "  ", &mut ElementFormatter::new(&mut s)).unwrap();
        s
    }
}

impl ImportReceiver for BTreeSet<ImportedName> {
    fn receive(&mut self, name: &ImportedName) {
        self.insert(name.clone());
    }
}
