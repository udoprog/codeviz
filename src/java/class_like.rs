use super::_type::AsClassType;
use super::annotation_spec::AsAnnotationSpec;
use super::class_spec::ClassSpec;
use super::constructor_spec::AsConstructorSpec;
use super::element_spec::AsElementSpec;
use super::enum_spec::EnumSpec;
use super::field_spec::{AsFieldSpec, FieldSpec};

pub trait ClassLike {
    fn implements<T>(&mut self, ty: T) where T: AsClassType;

    fn push_annotation<A>(&mut self, annotation: A) where A: AsAnnotationSpec;

    fn push_field<F>(&mut self, field: F) where F: AsFieldSpec;

    fn fields(&self) -> &Vec<FieldSpec>;

    fn push_constructor<C>(&mut self, constructor: C) where C: AsConstructorSpec;

    fn push<E>(&mut self, element: E) where E: AsElementSpec;
}

impl ClassLike for ClassSpec {
    fn implements<T>(&mut self, ty: T)
        where T: AsClassType
    {
        self.implements.push(ty.as_class_type());
    }

    fn push_annotation<A>(&mut self, annotation: A)
        where A: AsAnnotationSpec
    {
        self.annotations.push(annotation.as_annotation_spec());
    }

    fn push_field<F>(&mut self, field: F)
        where F: AsFieldSpec
    {
        self.fields.push(field.as_field_spec());
    }

    fn fields(&self) -> &Vec<FieldSpec> {
        &self.fields
    }

    fn push_constructor<C>(&mut self, constructor: C)
        where C: AsConstructorSpec
    {
        self.constructors.push(constructor.as_constructor_spec());
    }

    fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }
}

impl ClassLike for EnumSpec {
    fn implements<T>(&mut self, ty: T)
        where T: AsClassType
    {
        self.implements.push(ty.as_class_type());
    }

    fn push_annotation<A>(&mut self, annotation: A)
        where A: AsAnnotationSpec
    {
        self.annotations.push(annotation.as_annotation_spec());
    }

    fn push_field<F>(&mut self, field: F)
        where F: AsFieldSpec
    {
        self.fields.push(field.as_field_spec());
    }

    fn fields(&self) -> &Vec<FieldSpec> {
        &self.fields
    }

    fn push_constructor<C>(&mut self, constructor: C)
        where C: AsConstructorSpec
    {
        self.constructors.push(constructor.as_constructor_spec());
    }

    fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }
}
