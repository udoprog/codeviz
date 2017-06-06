use super::statement::Statement;
use super::_type::ClassType;

pub fn implements<'a, I>(implements: I, dest: &mut Statement)
    where I: IntoIterator<Item = &'a ClassType>
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
