use super::*;

pub fn implements<'a, I>(implements: I, dest: &mut Statement)
where
    I: IntoIterator<Item = &'a ClassType>,
{
    let mut it = implements.into_iter();

    if let Some(first) = it.next() {
        dest.push(" implements ");

        dest.push(first);

        while let Some(next) = it.next() {
            dest.push(", ");
            dest.push(next);
        }
    }
}

pub fn join_statements<Iter, Item, Sep>(statements: Iter, separator: Sep) -> Statement
where
    Iter: IntoIterator<Item = Item>,
    Item: Into<Statement>,
    Sep: Into<Variable> + Clone,
{
    let mut it = statements.into_iter();

    let first = if let Some(first) = it.next() {
        first
    } else {
        return Statement::new();
    };

    let mut s = Statement::new();

    s.push(first.into());

    while let Some(next) = it.next() {
        s.push(separator.clone().into());
        s.push(next.into());
    }

    s
}
