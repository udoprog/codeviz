use super::_type::ClassType;
use super::annotation_spec::AnnotationSpec;
use super::elements::Elements;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct InterfaceSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub elements: Elements,
    pub extends: Vec<ClassType>,
}

impl InterfaceSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> InterfaceSpec {
        InterfaceSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            elements: Elements::new(),
            extends: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn extends<T>(&mut self, ty: T)
        where T: Into<ClassType>
    {
        self.extends.push(ty.into());
    }
}
