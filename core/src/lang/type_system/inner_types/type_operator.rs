use crate::lang::invokable::operator::Operator;
use cupid_macros::InnerType;

#[derive(Clone, Debug, Copy, PartialEq, Eq, InnerType)]
#[cupid_type_id(102)]
pub struct TypeOperator(pub Operator);
