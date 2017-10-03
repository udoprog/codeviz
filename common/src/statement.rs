use super::*;

/// A single statement, made up by variables.
#[derive(Debug, Clone)]
pub struct Statement<Var>
where
    Var: VariableFormat,
{
    pub parts: Vec<Var>,
}

impl<Var> Statement<Var>
where
    Var: VariableFormat,
{
    pub fn new() -> Statement<Var> {
        Statement { parts: Vec::new() }
    }

    pub fn push<V>(&mut self, variable: V)
    where
        V: Into<Var>,
    {
        self.parts.push(variable.into());
    }

    pub fn join<A>(self, separator: A) -> Statement<Var>
    where
        A: Into<Var> + Clone,
    {
        let mut it = self.parts.into_iter();

        let part: Var = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<Var> = Vec::new();
        parts.push(part);

        while let Some(part) = it.next() {
            parts.push(separator.clone().into());
            parts.push(part);
        }

        Statement { parts: parts }
    }

    pub fn format<E>(&self, out: &mut E, depth: usize, extra: &mut Var::Extra) -> Result<()>
    where
        E: ElementFormat,
    {
        for part in &self.parts {
            part.format(out, depth, extra)?;
        }

        Ok(())
    }
}

impl<'a, Var> From<&'a str> for Statement<Var>
where
    Var: VariableFormat + From<&'a str>,
{
    fn from(value: &'a str) -> Statement<Var> {
        Statement { parts: vec![value.into()] }
    }
}

impl<Var> From<String> for Statement<Var>
where
    Var: VariableFormat + From<String>,
{
    fn from(value: String) -> Statement<Var> {
        Statement { parts: vec![value.into()] }
    }
}

impl<S, Var> From<Vec<S>> for Statement<Var>
where
    Var: VariableFormat,
    S: Into<Var>,
{
    fn from(values: Vec<S>) -> Statement<Var> {
        let mut s = Statement::new();

        for value in values {
            s.push(value.into());
        }

        s
    }
}

impl<Var> From<Statement<Var>> for Element<Var>
where
    Var: VariableFormat,
{
    fn from(value: Statement<Var>) -> Element<Var> {
        Element::Push(value)
    }
}
