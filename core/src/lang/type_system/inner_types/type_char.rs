use cupid_macros::{InnerType, InnerTypeEquals};

#[derive(Clone, Debug, Copy, PartialEq, Eq, InnerType, InnerTypeEquals)]
#[cupid_type_id(-10)]
pub struct TypeChar(pub char);
