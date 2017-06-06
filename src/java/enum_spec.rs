use super::_type::ClassType;
use super::annotation_spec::AnnotationSpec;
use super::constructor_spec::ConstructorSpec;
use super::element::Element;
use super::elements::Elements;
use super::field_spec::FieldSpec;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct EnumSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub values: Elements,
    pub fields: Vec<FieldSpec>,
    pub constructors: Vec<ConstructorSpec>,
    pub elements: Elements,
    pub implements: Vec<ClassType>,
}

impl EnumSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> EnumSpec {
        EnumSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            values: Elements::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            elements: Elements::new(),
            implements: Vec::new(),
        }
    }

    pub fn push_value<E>(&mut self, value: E)
        where E: Into<Element>
    {
        self.values.push(value);
    }
}
