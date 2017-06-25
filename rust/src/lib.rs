extern crate codeviz_common;

mod struct_spec;
mod enum_spec;
mod file_spec;
mod imports;
mod name;
mod variable;

pub use codeviz_common::Element::*;
pub use self::enum_spec::*;
pub use self::file_spec::*;
pub use self::imports::*;
pub use self::name::*;
pub use self::struct_spec::*;
pub use self::variable::*;
pub(crate) use self::errors::*;

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
