use super::*;

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

impl From<EnumSpec> for Element {
    fn from(value: EnumSpec) -> Element {
        let mut elements = Elements::new();

        for a in &value.annotations {
            elements.push(a);
        }

        // opening statement
        {
            let mut open = Statement::new();

            if !value.modifiers.is_empty() {
                open.push(value.modifiers);
                open.push(" ");
            }

            open.push("enum ");
            open.push(&value.name);

            implements(&value.implements, &mut open);

            open.push(" {");

            elements.push(open);
        }

        let mut enum_body = Elements::new();

        // enum values
        {
            let mut values = Elements::new();

            let mut value_joiner = Elements::new();

            let mut comma = Statement::new();
            comma.push(",");

            value_joiner.push(Concat(comma));

            values.push(value.values.join(value_joiner));

            let mut endl = Statement::new();
            endl.push(";");

            values.push(Concat(endl));

            enum_body.push(values);
        }

        if !value.fields.is_empty() {
            let mut fields = Elements::new();

            for field in &value.fields {
                let mut field: Statement = field.into();
                field.push(";");
                fields.push(field);
            }

            enum_body.push(fields);
        }

        for constructor in value.constructors {
            enum_body.push(constructor.as_element(&value.name));
        }

        for element in &value.elements.elements {
            enum_body.push(element);
        }

        elements.push_nested(enum_body.join(Spacing));
        elements.push("}");

        elements.into()
    }
}
