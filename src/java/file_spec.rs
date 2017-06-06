use errors::*;
use common::ElementFormatter;
use super::_type::ClassType;
use super::element::*;
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::statement::Statement;

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub package: String,
    pub elements: Elements,
}

impl FileSpec {
    pub fn new(package: &str) -> FileSpec {
        FileSpec {
            package: package.to_owned(),
            elements: Elements::new(),
        }
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<Element>
    {
        self.elements.push(element);
    }

    pub fn format<'a, W>(&'a self, out: &mut W) -> Result<()>
        where W: ::std::fmt::Write
    {
        let mut file = Elements::new();

        let mut package = Statement::new();
        package.push("package ");
        package.push(&self.package);
        package.push(";");

        file.push(package);

        let mut receiver: BTreeSet<ClassType> = BTreeSet::new();

        self.elements.imports(&mut receiver);

        let imports: BTreeSet<ClassType> = receiver.into_iter()
            .filter(|t| t.package != "java.lang")
            .filter(|t| t.package != self.package)
            .map(|t| t.to_raw())
            .collect();

        if !imports.is_empty() {
            let mut imported = Elements::new();

            for t in imports {
                let mut import = Statement::new();

                import.push("import ");
                import.push(t.package);
                import.push(".");
                import.push(t.name);
                import.push(";");

                imported.push(import);
            }

            file.push(imported);
        }

        let content: Element = self.elements.clone().join(Spacing).into();
        file.push(content);

        let file: Element = file.join(Spacing).into();

        file.format(&mut ElementFormatter::new(out))?;
        out.write_char('\n')?;

        Ok(())
    }
}

impl ImportReceiver for BTreeSet<ClassType> {
    fn receive(&mut self, ty: &ClassType) {
        self.insert(ty.clone());
    }
}

impl ToString for FileSpec {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut s).unwrap();
        s
    }
}
