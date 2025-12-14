use crate::lang::{
    invokable::{Invokable, operator::Operator},
    type_system::{Atom, InnerTypeTrait},
};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct TypeOperator(pub Operator);

impl InnerTypeTrait for TypeOperator {
    fn get_type() -> i16 {
        102
    }
}

impl From<Operator> for TypeOperator {
    fn from(value: Operator) -> Self {
        TypeOperator(value)
    }
}

impl From<Operator> for Atom<TypeOperator> {
    fn from(value: Operator) -> Self {
        Atom::new(TypeOperator(value))
    }
}

impl Invokable for TypeOperator {
    fn invoke_2(
        &self,
        _arg1: crate::lang::SuperType,
        _arg2: crate::lang::SuperType,
    ) -> Result<crate::lang::SuperType, crate::Error> {
        match self.0 {
            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Equals
            | Operator::NotEquals
            | Operator::Match
            | Operator::LessThan
            | Operator::UpTo
            | Operator::AtLeast
            | Operator::GreaterThan
            | Operator::Join
            | Operator::Fill
            | Operator::Take
            | Operator::Cut
            | Operator::Assign
            | Operator::And
            | Operator::Or => Err(crate::Error::NotYetImplemented),
        }
    }
}
