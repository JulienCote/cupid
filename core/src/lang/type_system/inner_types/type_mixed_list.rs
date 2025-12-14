use crate::{
    Error,
    lang::type_system::{InnerTypeTrait, SuperType},
};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeMixedList(pub Vec<SuperType>);

impl InnerTypeTrait for TypeMixedList {
    fn get_type() -> i16 {
        0
    }
}

impl From<Vec<SuperType>> for TypeMixedList {
    fn from(values: Vec<SuperType>) -> Self {
        TypeMixedList(values)
    }
}

impl TryFrom<SuperType> for TypeMixedList {
    type Error = Error;

    fn try_from(value: SuperType) -> Result<Self, Self::Error> {
        match value {
            SuperType::MixedList(v) => Ok(v),
            _ => Err(Error::Type),
        }
    }
}
