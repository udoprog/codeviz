use super::variable::Variable;

/// Quote a string to make it suitable as a literal Python string.
fn quote_string(input: &str) -> String {
    let mut out = String::new();
    let mut it = input.chars();

    out.push('"');

    while let Some(c) = it.next() {
        match c {
            '\t' => out.push_str("\\t"),
            '\u{0007}' => out.push_str("\\b"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{0014}' => out.push_str("\\f"),
            '\'' => out.push_str("\\'"),
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }

    out.push('"');
    out
}

/// A single statement, made up by variables.
#[derive(Debug, Clone)]
pub struct Statement {
    pub parts: Vec<Variable>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement { parts: Vec::new() }
    }

    pub fn push<V>(&mut self, variable: V)
        where V: Into<Variable>
    {
        self.parts.push(variable.into());
    }

    pub fn join<A>(self, separator: A) -> Statement
        where A: Into<Variable> + Clone
    {
        let mut it = self.parts.into_iter();

        let part: Variable = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<Variable> = Vec::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep.into());
            parts.push(part);
        }

        Statement { parts: parts }
    }

    pub fn format(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current: Vec<String> = Vec::new();

        for part in &self.parts {
            match *part {
                Variable::String(ref string) => {
                    current.push(quote_string(string));
                }
                Variable::Statement(ref stmt) => {
                    current.push(stmt.format().join(" "));
                }
                Variable::Literal(ref content) => {
                    current.push(content.to_owned());
                }
                Variable::Name(ref name) => {
                    current.push(name.format());
                }
            }
        }

        if !current.is_empty() {
            out.push(current.join(""));
            current.clear();
        }

        out
    }
}

impl<'a, T> From<&'a T> for Statement
    where T: Into<Statement> + Clone
{
    fn from(value: &'a T) -> Statement {
        value.clone().into()
    }
}

impl From<Variable> for Statement {
    fn from(value: Variable) -> Statement {
        Statement { parts: vec![value] }
    }
}

impl From<String> for Statement {
    fn from(value: String) -> Statement {
        Statement { parts: vec![Variable::Literal(value)] }
    }
}
