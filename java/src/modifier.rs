use std::collections::BTreeSet;
use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Modifier {
    Default,
    Public,
    Protected,
    Private,
    Abstract,
    Static,
    Final,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Modifiers {
    pub modifiers: BTreeSet<Modifier>,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers { modifiers: BTreeSet::new() }
    }

    pub fn insert(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }

    pub fn format(&self) -> String {
        let mut out: Vec<String> = Vec::new();

        for m in &self.modifiers {
            out.push(match *m {
                Modifier::Abstract => "abstract".to_owned(),
                Modifier::Default => "default".to_owned(),
                Modifier::Public => "public".to_owned(),
                Modifier::Protected => "protected".to_owned(),
                Modifier::Private => "private".to_owned(),
                Modifier::Static => "static".to_owned(),
                Modifier::Final => "final".to_owned(),
            });
        }

        out.join(" ")
    }

    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn contains(&self, modifier: &Modifier) -> bool {
        self.modifiers.contains(modifier)
    }
}

impl From<Modifiers> for Variable {
    fn from(value: Modifiers) -> Variable {
        Variable::Literal(value.format())
    }
}
