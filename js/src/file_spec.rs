use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map;
use super::*;
use codeviz_common::ElementFormatter;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub elements: Elements,
}

impl FileSpec {
    pub fn new() -> FileSpec {
        FileSpec { elements: Elements::new() }
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: Into<Element>,
    {
        self.elements.push(element);
    }

    fn module_to_path(&self, path: &str) -> String {
        let parts: Vec<&str> = path.split(".").collect();
        format!("{}.js", parts.join("/"))
    }

    fn imports(&self) -> Option<Elements> {
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

        if collected.is_empty() && wildcard.is_empty() {
            return None;
        }

        let mut out = Elements::new();

        for (module, names) in collected {
            let mut stmt = Statement::new();

            stmt.push("import ");
            stmt.push("{");
            stmt.push(names.join(", "));
            stmt.push("}");
            stmt.push(" from ");
            stmt.push(Variable::String(self.module_to_path(&module)));
            stmt.push(";");

            out.push(stmt);
        }

        for (module, alias) in wildcard {
            let mut stmt = Statement::new();

            stmt.push("import * as ");
            stmt.push(alias);
            stmt.push(" from ");
            stmt.push(Variable::String(self.module_to_path(&module)));
            stmt.push(";");

            out.push(stmt);
        }

        Some(out)
    }

    pub fn format<W>(&self, out: &mut W) -> Result<()>
    where
        W: ::std::fmt::Write,
    {
        let mut elements = Elements::new();

        if let Some(imports) = self.imports() {
            elements.push(imports);
        }

        elements.push(self.elements.clone().join(Spacing));

        let elements: Element = elements.join(Spacing).into();
        let mut extra = ();

        elements.format(&mut ElementFormatter::new(out), &mut extra)?;
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
