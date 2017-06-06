use super::_type::Type;
use super::annotation_spec::AnnotationSpec;
use super::modifier::Modifiers;
use super::statement::Statement;
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct ArgumentSpec {
    pub modifiers: Modifiers,
    pub ty: Type,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
}

impl ArgumentSpec {
    pub fn new<I>(modifiers: Modifiers, ty: I, name: &str) -> ArgumentSpec
        where I: Into<Type>
    {
        ArgumentSpec {
            modifiers: modifiers,
            ty: ty.into(),
            name: name.to_owned(),
            annotations: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }
}

impl<'a, A> From<&'a A> for ArgumentSpec
    where A: Into<ArgumentSpec> + Clone
{
    fn from(value: &'a A) -> ArgumentSpec {
        value.clone().into()
    }
}

impl From<ArgumentSpec> for Variable {
    fn from(value: ArgumentSpec) -> Variable {
        Variable::Literal(value.name)
    }
}

impl From<ArgumentSpec> for Statement {
    fn from(value: ArgumentSpec) -> Statement {
        let mut s = Statement::new();

        for a in &value.annotations {
            s.push(a);
            s.push(" ");
        }

        if !value.modifiers.is_empty() {
            s.push(value.modifiers);
            s.push(" ");
        }

        s.push(value.ty);
        s.push(" ");
        s.push(value.name);

        s
    }
}
