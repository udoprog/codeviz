use super::_type::ClassType;
use super::annotation_spec::AnnotationSpec;
use super::common::implements;
use super::constructor_spec::ConstructorSpec;
use super::element::*;
use super::elements::Elements;
use super::field_spec::FieldSpec;
use super::modifier::Modifiers;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub fields: Vec<FieldSpec>,
    pub constructors: Vec<ConstructorSpec>,
    pub elements: Elements,
    pub extends: Option<ClassType>,
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
            extends: None,
            implements: Vec::new(),
        }
    }

    pub fn extends<T>(&mut self, ty: T)
        where T: Into<ClassType>
    {
        self.extends = Some(ty.into());
    }
}

impl From<ClassSpec> for Element {
    fn from(value: ClassSpec) -> Element {
        let mut elements = Elements::new();

        for a in &value.annotations {
            elements.push(a);
        }

        let mut open = Statement::new();

        if !value.modifiers.is_empty() {
            open.push(value.modifiers);
            open.push(" ");
        }

        open.push("class ");
        open.push(&value.name);

        if let Some(ref extends) = value.extends {
            open.push(" extends ");
            open.push(extends);
        }

        implements(&value.implements, &mut open);

        open.push(" {");

        elements.push(open);

        let mut class_body = Elements::new();

        if !value.fields.is_empty() {
            let mut fields = Elements::new();

            for field in &value.fields {
                let mut field: Statement = field.into();
                field.push(";");
                fields.push(field);
            }

            class_body.push(fields);
        }

        for constructor in value.constructors {
            class_body.push(constructor.as_element(&value.name));
        }

        for element in &value.elements.elements {
            class_body.push(element);
        }

        elements.push_nested(class_body.join(Spacing));
        elements.push("}");

        elements.into()
    }
}

impl From<ClassType> for Variable {
    fn from(value: ClassType) -> Variable {
        Variable::Type(value.into())
    }
}
