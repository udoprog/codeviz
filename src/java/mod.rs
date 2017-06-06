/// A code generator inspired by JavaPoet (https://github.com/square/javapoet)

mod _type;
mod annotation_spec;
mod argument_spec;
mod class_like;
mod class_spec;
mod common;
mod constructor_spec;
mod container_spec;
mod element;
mod elements;
mod enum_spec;
mod field_spec;
mod file_spec;
mod imports;
mod interface_spec;
mod method_spec;
mod modifier;
mod statement;
mod variable;

pub use self::_type::*;
pub use self::annotation_spec::*;
pub use self::argument_spec::*;
pub use self::class_like::*;
pub use self::class_spec::*;
pub use self::constructor_spec::*;
pub use self::container_spec::*;
pub use self::element::*;
pub use self::elements::*;
pub use self::enum_spec::*;
pub use self::field_spec::*;
pub use self::file_spec::*;
pub use self::imports::*;
pub use self::interface_spec::*;
pub use self::method_spec::*;
pub use self::modifier::*;
pub use self::statement::*;
pub use self::variable::*;

#[derive(Debug, Clone)]
pub struct MethodArgument {
    pub modifiers: Modifiers,
}
