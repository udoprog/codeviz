use common::Element;
use common::Statement;
use super::class_spec::ClassSpec;
use super::elements::Elements;
use super::name::{Name, ImportedName};
use super::variable::Variable;

pub trait ImportReceiver {
    fn receive(&mut self, name: &ImportedName);

    fn import_all<T>(&mut self, sources: &Vec<T>)
        where T: Imports,
              Self: Sized
    {
        for source in sources {
            source.imports(self);
        }
    }
}

pub trait Imports {
    fn imports<I>(&self, receiver: &mut I) where I: ImportReceiver;
}

impl Imports for Name {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Name::Imported(ref imported) => receiver.receive(imported),
            _ => {}
        };
    }
}

impl Imports for Variable {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Variable::Statement(ref stmt) => {
                stmt.imports(receiver);
            }
            Variable::Name(ref name) => {
                name.imports(receiver);
            }
            _ => {}
        }
    }
}

impl Imports for Statement<Variable> {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.parts);
    }
}

impl Imports for Element<Variable> {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Element::Statement(ref statement) => {
                statement.imports(receiver);
            }
            Element::Elements(ref elements) => {
                receiver.import_all(elements);
            }
            Element::Nested(ref element) => {
                element.imports(receiver);
            }
            _ => {}
        };
    }
}

impl Imports for ClassSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.elements.imports(receiver);
    }
}

impl Imports for Elements {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.elements);
    }
}
