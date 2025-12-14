use cupid_macros::{InnerType, InnerTypeArithmetic, InnerTypeEquals};

#[derive(Clone, Debug, Copy, PartialEq, InnerType, InnerTypeArithmetic, InnerTypeEquals)]
#[cupid_type_id(-9)]
pub struct TypeFloat(pub f64);
