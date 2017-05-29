use common::ElementFormat;
use super::class_spec::ClassSpec;
use super::elements::Elements;
use super::function_spec::FunctionSpec;
use super::method_spec::MethodSpec;
use super::constructor_spec::ConstructorSpec;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum ElementSpec {
    Statement(Statement),
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
            ElementSpec::Statement(ref statement) => {
                for line in statement.format() {
                    out.push(&format!("{}{}", current, line));
                }
            }
            ElementSpec::Literal(ref line) => {
                out.push(&format!("{}{}", current, line));
            }
            ElementSpec::Elements(ref elements) => {
                for element in elements {
                    element.format(current, indent, out);
                }
            }
            ElementSpec::Nested(ref element) => {
                let next_current = format!("{}{}", current, indent);
                element.format(&next_current, indent, out);
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

impl From<FunctionSpec> for ElementSpec {
    fn from(value: FunctionSpec) -> ElementSpec {
        let mut open = Statement::new();
        open.push("function ");
        open.push(value.name);
        open.push("(");

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        open.push(arguments.join(", "));
        open.push(") {");

        let mut out = Elements::new();
        out.push(open);
        out.push_nested(value.elements.join(ElementSpec::Spacing));
        out.push("}");

        out.into()
    }
}

impl From<ClassSpec> for ElementSpec {
    fn from(value: ClassSpec) -> ElementSpec {
        let mut open = Statement::new();

        open.push("class ");
        open.push(value.name);
        open.push(" {");

        let mut body = Elements::new();

        if !value.constructors.is_empty() {
            body.push_nested(value.constructors.join(ElementSpec::Spacing));
        }

        if !value.elements.is_empty() {
            body.push_nested(value.elements.join(ElementSpec::Spacing));
        }

        let mut out = Elements::new();
        out.push(open);
        out.push_nested(body.join(ElementSpec::Spacing));
        out.push("}");

        out.into()
    }
}

impl From<MethodSpec> for ElementSpec {
    fn from(value: MethodSpec) -> ElementSpec {
        let mut open = Statement::new();

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        if value.is_static {
            open.push("static ");
        }

        open.push(value.name);
        open.push("(");
        open.push(arguments.join(", "));
        open.push(")");

        open.push(" {");

        let mut out = Elements::new();
        out.push(open);
        out.push_nested(value.elements.join(ElementSpec::Spacing));
        out.push("}");

        out.into()
    }
}

impl From<ConstructorSpec> for ElementSpec {
    fn from(value: ConstructorSpec) -> ElementSpec {
        let mut open = Statement::new();

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        open.push("constructor(");
        open.push(arguments.join(", "));
        open.push(")");

        open.push(" {");

        let mut out = Elements::new();

        out.push(open);
        out.push_nested(value.elements.join(ElementSpec::Spacing));
        out.push("}");

        out.into()
    }
}

impl From<Statement> for ElementSpec {
    fn from(value: Statement) -> ElementSpec {
        ElementSpec::Statement(value)
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
