use super::_type::Type;
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub returns: Option<Type>,
    pub elements: Elements,
}

impl MethodSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> MethodSpec {
        MethodSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            arguments: Vec::new(),
            returns: None,
            elements: Elements::new(),
        }
    }

    pub fn push_annotation<A>(&mut self, annotation: A)
        where A: Into<AnnotationSpec>
    {
        self.annotations.push(annotation.into());
    }

    pub fn push_argument<A>(&mut self, argument: A)
        where A: Into<ArgumentSpec>
    {
        self.arguments.push(argument.into());
    }

    pub fn returns<T>(&mut self, returns: T)
        where T: Into<Type>
    {
        self.returns = Some(returns.into())
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
    }
}
