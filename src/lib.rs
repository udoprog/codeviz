#[macro_use]
extern crate error_chain;

pub mod common;
pub mod errors;
pub mod java;
pub mod js;
pub mod python;
pub mod rust;

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

        let reference = ::std::str::from_utf8(include_bytes!("tests/test.py")).unwrap();
        assert_eq!(reference, file.to_string());
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

        println!("result = {}", file.to_string());

        let reference = ::std::str::from_utf8(include_bytes!("tests/Test.java")).unwrap();
        assert_eq!(reference, file.to_string());
    }

    #[test]
    fn test_class_spec() {
        let class = ClassSpec::new(mods![Modifier::Public], "Foo");
        let class: Element = class.into();
        assert_eq!("public class Foo {\n}", class.to_string());
    }

    #[test]
    fn test_interface_spec() {
        let class = InterfaceSpec::new(mods![Modifier::Public], "Foo");
        let class: Element = class.into();
        assert_eq!("public interface Foo {\n}", class.to_string());
    }

    #[test]
    fn test_enum_spec() {
        let class = EnumSpec::new(mods![Modifier::Public], "Foo");
        let class: Element = class.into();
        assert_eq!("public enum Foo {\n  ;\n}", class.to_string());
    }
}

#[cfg(test)]
mod js_tests {
    use js::*;

    #[test]
    fn test_file() {
        let mut foo = FunctionSpec::new("foo");
        let m = Name::imported("foo", "hello");
        foo.push(stmt!["return ", m, "();"]);

        let mut file = FileSpec::new();
        file.push(foo);

        let result = file.to_string();

        assert_eq!("import {hello} from \"foo.js\";\n\nfunction foo() {\n  return hello();\n}\n",
                   result);
    }
}
