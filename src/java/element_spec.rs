use super::annotation_spec::AnnotationSpec;
use super::class_spec::ClassSpec;
use super::elements::Elements;
use super::enum_spec::EnumSpec;
use super::interface_spec::InterfaceSpec;
use super::method_spec::MethodSpec;
use super::statement::Statement;

pub trait ElementFormat {
    fn push(&mut self, value: &str);

    fn concat(&mut self, value: &str);
}

#[derive(Debug, Clone)]
pub enum ElementSpec {
    // push as individual line.
    Push(Statement),
    // concat to previous statement.
    Concat(Statement),
    Literal(String),
    Elements(Vec<ElementSpec>),
    Nested(Box<ElementSpec>),
    Spacing,
}

impl ElementSpec {
    pub fn format<E>(&self, current: &str, indent: &str, out: &mut E)
        where E: ElementFormat
    {
        match *self {
            ElementSpec::Push(ref statement) => {
                for line in statement.format(0usize) {
                    out.push(&format!("{}{}", current, line));
                }
            }
            ElementSpec::Concat(ref statement) => {
                for line in statement.format(0usize) {
                    out.concat(&line);
                }
            }
            ElementSpec::Literal(ref line) => {
                out.push(&format!("{}{}", current, line));
            }
            ElementSpec::Elements(ref elements) => {
                for element in elements {
                    element.format(current, indent, out)
                }
            }
            ElementSpec::Nested(ref element) => {
                let next_current = format!("{}{}", current, indent);
                element.format(&next_current, indent, out)
            }
            ElementSpec::Spacing => {
                out.push("");
            }
        };
    }
}

impl<'a, T> From<&'a T> for ElementSpec
    where T: Into<ElementSpec> + Clone
{
    fn from(value: &'a T) -> ElementSpec {
        value.clone().into()
    }
}

impl<'a> From<&'a str> for ElementSpec {
    fn from(value: &'a str) -> ElementSpec {
        ElementSpec::Literal(value.to_owned())
    }
}

impl From<Statement> for ElementSpec {
    fn from(value: Statement) -> ElementSpec {
        ElementSpec::Push(value)
    }
}

impl From<Elements> for ElementSpec {
    fn from(value: Elements) -> ElementSpec {
        ElementSpec::Elements(value.elements)
    }
}

impl From<Vec<String>> for ElementSpec {
    fn from(value: Vec<String>) -> ElementSpec {
        ElementSpec::Elements(value.into_iter().map(ElementSpec::Literal).collect())
    }
}

impl From<ClassSpec> for ElementSpec {
    fn from(value: ClassSpec) -> ElementSpec {
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

        if !value.implements.is_empty() {
            let mut arguments = Statement::new();

            for implements in &value.implements {
                arguments.push(implements);
            }

            open.push(" implements ");
            open.push(arguments.join(", "));
        }

        open.push(" {");

        elements.push(open);

        let mut class_body = Elements::new();

        let mut fields = Elements::new();

        for field in &value.fields {
            let mut field: Statement = field.into();
            field.push(";");
            fields.push(field);
        }

        class_body.push(fields);

        for constructor in &value.constructors {
            class_body.push(constructor.as_element_spec(&value.name));
        }

        for element in &value.elements.elements {
            class_body.push(element);
        }

        elements.push_nested(class_body.join(ElementSpec::Spacing));
        elements.push("}");

        elements.into()
    }
}

impl From<MethodSpec> for ElementSpec {
    fn from(value: MethodSpec) -> ElementSpec {
        let mut elements = Elements::new();

        for a in &value.annotations {
            elements.push(a);
        }

        let mut open = Statement::new();

        if !value.modifiers.is_empty() {
            open.push(value.modifiers);
            open.push(" ");
        }

        match value.returns {
            None => open.push("void "),
            Some(ref returns) => {
                open.push(returns);
                open.push(" ");
            }
        }

        open.push(value.name);
        open.push("(");

        if !value.arguments.is_empty() {
            open.push(Statement::join_statements(&value.arguments, ", "));
        }

        open.push(") {");

        elements.push(open);
        elements.push_nested(value.elements.join(ElementSpec::Spacing));
        elements.push("}");

        elements.into()
    }
}

impl From<InterfaceSpec> for ElementSpec {
    fn from(value: InterfaceSpec) -> ElementSpec {
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
        elements.push_nested(value.elements.join(ElementSpec::Spacing));
        elements.push("}");

        elements.into()
    }
}

impl From<AnnotationSpec> for ElementSpec {
    fn from(value: AnnotationSpec) -> ElementSpec {
        let mut elements = Elements::new();

        let mut annotation = Statement::new();
        annotation.push("@");
        annotation.push(value.ty);

        if !value.arguments.is_empty() {
            let mut open = Statement::new();

            open.push(annotation);
            open.push("(");
            open.push(Statement::join_with(&value.arguments, ", "));
            open.push(")");

            elements.push(open);
        } else {
            elements.push(annotation);
        }

        elements.into()
    }
}

impl From<EnumSpec> for ElementSpec {
    fn from(value: EnumSpec) -> ElementSpec {
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

            if !value.implements.is_empty() {
                let mut arguments = Statement::new();

                for implements in &value.implements {
                    arguments.push(implements);
                }

                open.push(" implements ");
                open.push(arguments.join(","));
            }

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

            value_joiner.push(ElementSpec::Concat(comma));

            values.push(value.values.join(value_joiner));

            let mut endl = Statement::new();
            endl.push(";");

            values.push(ElementSpec::Concat(endl));

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

        for constructor in &value.constructors {
            enum_body.push(constructor.as_element_spec(&value.name));
        }

        for element in &value.elements.elements {
            enum_body.push(element);
        }

        elements.push_nested(enum_body.join(ElementSpec::Spacing));
        elements.push("}");

        elements.into()
    }
}
