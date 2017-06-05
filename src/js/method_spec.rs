use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub name: String,
    pub arguments: Vec<Statement>,
    pub elements: Elements,
    pub is_static: bool,
}

impl MethodSpec {
    pub fn new(name: &str) -> MethodSpec {
        MethodSpec {
            name: name.to_owned(),
            arguments: Vec::new(),
            elements: Elements::new(),
            is_static: false,
        }
    }

    pub fn with_static(name: &str) -> MethodSpec {
        MethodSpec {
            name: name.to_owned(),
            arguments: Vec::new(),
            elements: Elements::new(),
            is_static: true,
        }
    }

    pub fn push_argument<S>(&mut self, argument: S)
        where S: Into<Statement>
    {
        self.arguments.push(argument.into());
    }

    pub fn push<E>(&mut self, element: E)
        where E: Into<ElementSpec>
    {
        self.elements.push(element);
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