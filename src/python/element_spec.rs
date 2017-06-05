use common::ElementFormat;
use errors::*;
use super::class_spec::ClassSpec;
use super::decorator_spec::DecoratorSpec;
use super::elements::Elements;
use super::method_spec::MethodSpec;
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
    pub fn format<E>(&self, out: &mut E) -> Result<()>
        where E: ElementFormat
    {
        match *self {
            ElementSpec::Statement(ref statement) => {
                out.new_line_unless_empty()?;
                statement.format(out)?;
            }
            ElementSpec::Literal(ref line) => {
                out.new_line_unless_empty()?;
                out.write_str(line)?;
            }
            ElementSpec::Elements(ref elements) => {
                for element in elements {
                    element.format(out)?;
                }
            }
            ElementSpec::Nested(ref element) => {
                out.new_line_unless_empty()?;

                out.indent();
                element.format(out)?;
                out.unindent();
            }
            ElementSpec::Spacing => {
                out.new_line_unless_empty()?;
                out.new_line()?;
            }
        }

        Ok(())
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

impl From<MethodSpec> for ElementSpec {
    fn from(value: MethodSpec) -> ElementSpec {
        let mut out: Vec<ElementSpec> = Vec::new();

        for decorator in value.decorators {
            out.push(decorator.into());
        }

        let mut decl = Statement::new();
        decl.push("def ");
        decl.push(value.name);
        decl.push("(");

        let mut arguments = Statement::new();

        for argument in value.arguments {
            arguments.push(argument);
        }

        decl.push(arguments.join(", "));
        decl.push("):");

        out.push(decl.into());

        if value.elements.is_empty() {
            out.push(ElementSpec::Nested(Box::new("pass".into())));
        } else {
            out.push(ElementSpec::Nested(Box::new(value.elements.into())));
        }

        ElementSpec::Elements(out)
    }
}

impl From<ClassSpec> for ElementSpec {
    fn from(value: ClassSpec) -> ElementSpec {
        let mut out = Elements::new();

        for decorator in value.decorators {
            out.push(decorator);
        }

        let mut decl = Statement::new();
        decl.push("class ");
        decl.push(value.name);

        if !value.extends.is_empty() {
            decl.push("(");

            let mut extends = Statement::new();

            for extend in value.extends {
                extends.push(extend);
            }

            decl.push(extends.join(", "));
            decl.push(")");
        }

        decl.push(":");

        out.push(decl);

        if value.elements.is_empty() {
            out.push_nested("pass");
        } else {
            out.push_nested(value.elements.join(ElementSpec::Spacing));
        }

        out.into()
    }
}

impl From<DecoratorSpec> for ElementSpec {
    fn from(value: DecoratorSpec) -> ElementSpec {
        let mut decl = Statement::new();

        decl.push("@");
        decl.push(value.name);

        decl.into()
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

impl ToString for ElementSpec {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.format(&mut ::common::ElementFormatter::new(&mut s)).unwrap();
        s
    }
}
