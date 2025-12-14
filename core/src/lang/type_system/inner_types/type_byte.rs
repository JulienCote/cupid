use cupid_macros::{InnerType, InnerTypeArithmetic, InnerTypeEquals};
use super::TypeInt;

#[derive(Clone, Debug, Copy, PartialEq, Eq, InnerType, InnerTypeArithmetic, InnerTypeEquals)]
#[cupid_type_id(-4)]
#[cupid_arithmetic_promote(TypeInt, i32)]
pub struct TypeByte(pub u8);
