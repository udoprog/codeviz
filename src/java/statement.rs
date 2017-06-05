use common::ElementFormat;
use errors::*;
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::field_spec::FieldSpec;
use super::variable::Variable;

fn java_quote_string(out: &mut ElementFormat, input: &str) -> Result<()> {
    out.write_char('"')?;

    for c in input.chars() {
        match c {
            '\t' => out.write_str("\\t")?,
            '\u{0007}' => out.write_str("\\b")?,
            '\n' => out.write_str("\\n")?,
            '\r' => out.write_str("\\r")?,
            '\u{0014}' => out.write_str("\\f")?,
            '\'' => out.write_str("\\'")?,
            '"' => out.write_str("\\\"")?,
            '\\' => out.write_str("\\\\")?,
            c => out.write_char(c)?,
        }
    }

    out.write_char('"')?;

    Ok(())
}

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

    pub fn join<A>(&self, separator: A) -> Statement
        where A: Into<Variable> + Clone
    {
        Statement::join_with(&self.parts, separator)
    }

    pub fn join_with<'a, I, S, A>(parts: I, separator: A) -> Statement
        where I: IntoIterator<Item = &'a S>,
              S: 'a + Into<Variable> + Clone,
              A: Into<Variable> + Clone
    {
        let mut it = parts.into_iter().map(Into::into);

        let part = match it.next() {
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

    pub fn join_statements<'a, I, S, A>(parts: I, separator: A) -> Statement
        where I: IntoIterator<Item = &'a S>,
              S: 'a + Into<Statement> + Clone,
              A: Into<Variable> + Clone
    {
        let mut it = parts.into_iter().map(Into::into);

        let part: Statement = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut stmt = Statement::new();
        stmt.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            stmt.push(sep);
            stmt.push(part);
        }

        stmt
    }

    pub fn format(&self, out: &mut ElementFormat, level: usize) -> Result<()> {
        for part in &self.parts {
            match *part {
                Variable::Type(ref ty) => ty.format(out, level)?,
                Variable::String(ref string) => java_quote_string(out, string)?,
                Variable::Statement(ref stmt) => stmt.format(out, level)?,
                Variable::Literal(ref content) => out.write_str(content)?,
                Variable::Spacing => out.new_line()?,
            }
        }

        Ok(())
    }
}

impl<'a, T> From<&'a T> for Statement
    where T: Into<Statement> + Clone
{
    fn from(value: &'a T) -> Statement {
        value.clone().into()
    }
}

impl From<FieldSpec> for Statement {
    fn from(value: FieldSpec) -> Statement {
        let mut s = Statement::new();

        if !value.modifiers.is_empty() {
            s.push(value.modifiers);
            s.push(" ");
        }

        s.push(value.ty);
        s.push(" ");
        s.push(value.name);

        if let Some(initialize) = value.initialize {
            s.push(" = ");
            s.push(initialize);
        }

        s
    }
}

impl From<ArgumentSpec> for Statement {
    fn from(value: ArgumentSpec) -> Statement {
        let mut s = Statement::new();

        for a in &value.annotations {
            s.push(a);
            s.push(" ");
        }

        if !value.modifiers.is_empty() {
            s.push(value.modifiers);
            s.push(" ");
        }

        s.push(value.ty);
        s.push(" ");
        s.push(value.name);

        s
    }
}

impl From<AnnotationSpec> for Statement {
    fn from(value: AnnotationSpec) -> Statement {
        let mut stmt = Statement::new();

        let mut annotation = Statement::new();
        annotation.push("@");
        annotation.push(value.ty);

        if !value.arguments.is_empty() {
            stmt.push(annotation);
            stmt.push("(");
            stmt.push(Statement::join_with(&value.arguments, ", "));
            stmt.push(")");
        } else {
            stmt.push(annotation);
        }

        stmt
    }
}

impl From<Variable> for Statement {
    fn from(value: Variable) -> Statement {
        let mut stmt = Statement::new();
        stmt.push(value);
        stmt
    }
}
