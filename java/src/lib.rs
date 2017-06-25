/// A code generator inspired by JavaPoet (https://github.com/square/javapoet)
extern crate codeviz_common;

mod _type;
mod annotation_spec;
mod argument_spec;
mod class_like;
mod class_spec;
mod common;
mod constructor_spec;
mod container_spec;
mod enum_spec;
mod extra;
mod field_spec;
mod file_spec;
mod imports;
mod interface_spec;
mod method_argument;
mod method_spec;
mod modifier;
mod variable;

pub use codeviz_common::Element::*;
pub use self::_type::*;
pub use self::annotation_spec::*;
pub use self::argument_spec::*;
pub use self::class_like::*;
pub use self::class_spec::*;
pub use self::constructor_spec::*;
pub use self::container_spec::*;
pub use self::enum_spec::*;
pub use self::field_spec::*;
pub use self::file_spec::*;
pub use self::imports::*;
pub use self::interface_spec::*;
pub use self::method_argument::*;
pub use self::method_spec::*;
pub use self::modifier::*;
pub use self::variable::*;
pub(crate) use self::common::*;
pub(crate) use self::errors::*;
pub(crate) use self::extra::*;

pub mod errors {
    pub use codeviz_common::errors::*;
}

pub type Element = codeviz_common::Element<Variable>;
pub type Elements = codeviz_common::Elements<Variable>;
pub type Statement = codeviz_common::Statement<Variable>;

impl From<Variable> for Statement {
    fn from(value: Variable) -> Statement {
        Statement { parts: vec![value] }
    }
}
