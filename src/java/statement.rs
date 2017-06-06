use common;
use super::variable::Variable;

pub type Statement = common::Statement<Variable>;

impl From<String> for Statement {
    fn from(value: String) -> Statement {
        Statement { parts: vec![Variable::Literal(value)] }
    }
}

impl From<Variable> for Statement {
    fn from(value: Variable) -> Statement {
        Statement { parts: vec![value] }
    }
}
