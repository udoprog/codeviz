use super::_type::ClassType;
use super::annotation_spec::AnnotationSpec;
use super::constructor_spec::ConstructorSpec;
use super::elements::Elements;
use super::field_spec::FieldSpec;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub fields: Vec<FieldSpec>,
    pub constructors: Vec<ConstructorSpec>,
    pub elements: Elements,
    pub implements: Vec<ClassType>,
}

impl ClassSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> ClassSpec {
        ClassSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            elements: Elements::new(),
            implements: Vec::new(),
        }
    }
}
