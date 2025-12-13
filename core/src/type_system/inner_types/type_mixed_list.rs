use crate::{
    Error,
    type_system::{Atom, InnerTypeTrait, List, SuperType},
};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeMixedList(pub List<SuperType>);

impl From<List<SuperType>> for TypeMixedList {
    fn from(value: List<SuperType>) -> Self {
        TypeMixedList(value)
    }
}

impl From<List<SuperType>> for Atom<TypeMixedList> {
    fn from(value: List<SuperType>) -> Self {
        Atom::new(TypeMixedList(value))
    }
}

impl InnerTypeTrait for TypeMixedList {
    fn get_type() -> i16 {
        0
    }
}

impl From<Vec<SuperType>> for TypeMixedList {
    fn from(values: Vec<SuperType>) -> Self {
        TypeMixedList(List::new(values))
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
