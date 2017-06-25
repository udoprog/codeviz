use super::*;

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

impl From<InterfaceSpec> for Element {
    fn from(value: InterfaceSpec) -> Element {
        let mut elements = Elements::new();

        let mut open = Statement::new();

        for a in &value.annotations {
            elements.push(a);
        }

        if !value.modifiers.is_empty() {
            open.push(value.modifiers);
            open.push(" ");
        }

        open.push("interface ");
        open.push(value.name);

        if !value.extends.is_empty() {
            let mut arguments = Statement::new();

            for extends in &value.extends {
                arguments.push(extends);
            }

            open.push(" extends ");
            open.push(arguments.join(","));
        }

        open.push(" {");

        elements.push(open);
        elements.push_nested(value.elements.join(Spacing));
        elements.push("}");

        elements.into()
    }
}
