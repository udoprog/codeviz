/// Macro to build statements.
///
/// # Examples
///
/// All of these are equivalent:
///
/// ```
/// #[macro_use]
/// extern crate codeviz_macros;
/// extern crate codeviz_java;
///
/// use codeviz_java::*;
///
/// pub fn main() {
///   let stmt1 = stmt![Variable::Literal("hello".to_owned())];
///
///   let stmt2 = stmt!["hello"];
///
///   let stmt3 = {
///     let mut s = Statement::new();
///     s.push("hello");
///     s
///   };
/// }
/// ```
#[macro_export]
macro_rules! stmt {
    ($($var:expr),*) => {{
        let mut statement = Statement::new();
        $(statement.push($var);)*
        statement
    }};

    ($($var:expr,)*) => {{
        let mut statement = Statement::new();
        $(statement.push($var);)*
        statement
    }};
}

/// Build a modifier list.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate codeviz_macros;
/// extern crate codeviz_java;
///
/// use codeviz_java::*;
///
/// pub fn main() {
///   let mods = mods![Modifier::Private, Modifier::Static];
/// }
/// ```
#[macro_export]
macro_rules! mods {
    ($($modifier:expr),*) => {{
        let mut modifiers = Modifiers::new();
        $(modifiers.insert($modifier);)*
        modifiers
    }};

    ($($modifier:expr,)*) => {{
        let mut modifiers = Modifiers::new();
        $(modifiers.insert($modifier);)*
        modifiers
    }};
}
