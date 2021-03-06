use super::*;

pub trait ImportReceiver {
    fn receive(&mut self, name: &ImportedName);

    fn import_all<T>(&mut self, sources: &Vec<T>)
    where
        T: Imports,
        Self: Sized,
    {
        for source in sources {
            source.imports(self);
        }
    }
}

pub trait Imports {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver;
}

impl Imports for Name {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        match *self {
            Name::Imported(ref imported) => receiver.receive(imported),
            _ => {}
        };
    }
}

impl Imports for Variable {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
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

impl Imports for Statement {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        receiver.import_all(&self.parts);
    }
}

impl Imports for Element {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        match *self {
            Push(ref statement) => {
                statement.imports(receiver);
            }
            Inner(ref elements) => {
                receiver.import_all(elements);
            }
            Nested(ref element) => {
                element.imports(receiver);
            }
            _ => {}
        };
    }
}

impl Imports for FunctionSpec {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        receiver.import_all(&self.arguments);
        self.elements.imports(receiver);
    }
}

impl Imports for Elements {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        receiver.import_all(&self.elements);
    }
}
