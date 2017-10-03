use super::*;

pub trait ImportReceiver {
    fn receive(&mut self, ty: &ClassType);

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
    fn imports<I>(&self, &mut I)
    where
        I: ImportReceiver;
}

impl Imports for Element {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        match *self {
            Inner(ref elements) => receiver.import_all(elements),
            Push(ref statement) => statement.imports(receiver),
            Concat(ref statement) => statement.imports(receiver),
            Nested(ref nested) => {
                (*nested).imports(receiver);
            }
            _ => {}
        }
    }
}

impl Imports for Variable {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        match *self {
            Variable::Type(ref ty) => {
                ty.imports(receiver);
            }
            Variable::Statement(ref stmt) => {
                stmt.imports(receiver);
            }
            Variable::Element(ref element) => {
                element.imports(receiver);
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

impl Imports for Elements {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        receiver.import_all(&self.elements);
    }
}

impl Imports for ClassType {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        receiver.receive(self);
        receiver.import_all(&self.arguments);
    }
}

impl Imports for Type {
    fn imports<I>(&self, receiver: &mut I)
    where
        I: ImportReceiver,
    {
        match *self {
            Type::Class(ref class) => class.imports(receiver),
            _ => {}
        };
    }
}
