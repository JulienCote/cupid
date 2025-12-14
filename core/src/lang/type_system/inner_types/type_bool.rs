use super::TypeInt;
use cupid_macros::{InnerType, InnerTypeArithmetic, InnerTypeEquals};

#[derive(Clone, Debug, Copy, PartialEq, Eq, InnerType, InnerTypeArithmetic, InnerTypeEquals)]
#[cupid_type_id(-1)]
#[cupid_arithmetic_promote(TypeInt, i32)]
pub struct TypeBool(pub bool);
