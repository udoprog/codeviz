use super::*;

pub trait ClassLike {
    fn implements<T>(&mut self, ty: T) where T: Into<ClassType>;

    fn push_annotation<A>(&mut self, annotation: A) where A: Into<AnnotationSpec>;

    fn push_field<F>(&mut self, field: F) where F: Into<FieldSpec>;

    fn fields(&self) -> &Vec<FieldSpec>;

    fn push_constructor<C>(&mut self, constructor: C) where C: Into<ConstructorSpec>;
}

impl ClassLike for ClassSpec {
    fn implements<T>(&mut self, ty: T)
        where T: Into<ClassType>
    {
        self.implements.push(ty.into());
    }

    fn push_annotation<A>(&mut self, annotation: A)
        where A: Into<AnnotationSpec>
    {
        self.annotations.push(annotation.into());
    }

    fn push_field<F>(&mut self, field: F)
        where F: Into<FieldSpec>
    {
        self.fields.push(field.into());
    }

    fn fields(&self) -> &Vec<FieldSpec> {
        &self.fields
    }

    fn push_constructor<C>(&mut self, constructor: C)
        where C: Into<ConstructorSpec>
    {
        self.constructors.push(constructor.into());
    }
}

impl ClassLike for EnumSpec {
    fn implements<T>(&mut self, ty: T)
        where T: Into<ClassType>
    {
        self.implements.push(ty.into());
    }

    fn push_annotation<A>(&mut self, annotation: A)
        where A: Into<AnnotationSpec>
    {
        self.annotations.push(annotation.into());
    }

    fn push_field<F>(&mut self, field: F)
        where F: Into<FieldSpec>
    {
        self.fields.push(field.into());
    }

    fn fields(&self) -> &Vec<FieldSpec> {
        &self.fields
    }

    fn push_constructor<C>(&mut self, constructor: C)
        where C: Into<ConstructorSpec>
    {
        self.constructors.push(constructor.into());
    }
}
