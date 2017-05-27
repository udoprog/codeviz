#[macro_use]
extern crate error_chain;

pub mod errors;
pub mod java;
pub mod python;

/// Macro to build statements.
///
/// # Examples
///
/// All of these are equivalent:
///
/// ```
/// #[macro_use]
/// extern crate codeviz;
///
/// use codeviz::java::*;
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
}

/// Build a modifier list.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate codeviz;
///
/// use codeviz::java::*;
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
    }}
}

#[cfg(test)]
mod python_tests {
    use python::*;

    #[test]
    fn test_python() {
        let static_method = Name::built_in("staticmethod");
        let exit = Name::imported("sys", "exit");

        let mut file = FileSpec::new();

        let mut hello = MethodSpec::new("hello");
        hello.push_decorator(static_method);
        hello.push(stmt!["return 12"]);

        let mut bye = MethodSpec::new("bye");
        bye.push(stmt![exit, "(1)"]);

        let mut foo = ClassSpec::new("Foo");
        foo.push(hello);
        foo.push(bye);

        file.push(foo);

        let result = file.format();

        let reference = ::std::str::from_utf8(include_bytes!("tests/test.py")).unwrap();
        assert_eq!(reference, result);
    }
}


#[cfg(test)]
mod java_tests {
    use java::*;

    #[test]
    fn test_test_java() {
        let string_type = Type::class("java.lang", "String");
        let list_type = Type::class("java.util", "List");
        let json_creator_type = Type::class("com.fasterxml.jackson.annotation", "JsonCreator");
        let list_of_strings = list_type.with_arguments(vec![&string_type]);

        let values_field = FieldSpec::new(mods![Modifier::Private, Modifier::Final],
                                          &list_of_strings,
                                          "values");

        let values_argument = ArgumentSpec::new(mods![Modifier::Final], &list_of_strings, "values");

        let mut constructor = ConstructorSpec::new(mods![Modifier::Public]);
        constructor.push_annotation(AnnotationSpec::new(json_creator_type));
        constructor.push_argument(&values_argument);
        constructor.push(stmt!["this.values = ", values_argument, ";"]);

        let mut values_getter = MethodSpec::new(mods![Modifier::Public], "getValues");
        values_getter.returns(&list_of_strings);
        values_getter.push(stmt!["return this.", &values_field, ";"]);

        let mut class = ClassSpec::new(mods![Modifier::Public], "Test");
        class.push_field(&values_field);
        class.push_constructor(&constructor);
        class.push(&values_getter);

        let mut file = FileSpec::new("se.tedro");
        file.push(&class);

        let result = file.format();

        println!("{}", result);

        let reference = ::std::str::from_utf8(include_bytes!("tests/Test.java")).unwrap();
        assert_eq!(reference, result);
    }
}
