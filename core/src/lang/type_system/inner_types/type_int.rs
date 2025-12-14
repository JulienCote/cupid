use cupid_macros::{InnerType, InnerTypeArithmetic, InnerTypeEquals};

#[derive(Clone, Debug, Copy, PartialEq, Eq, InnerType, InnerTypeArithmetic, InnerTypeEquals)]
#[cupid_type_id(-6)]
pub struct TypeInt(pub i32);
