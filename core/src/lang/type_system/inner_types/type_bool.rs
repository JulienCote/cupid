use crate::lang::type_system::{Atom, InnerTypeTrait, List};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct TypeBool(pub bool);

impl From<bool> for TypeBool {
    fn from(value: bool) -> Self {
        TypeBool(value)
    }
}

impl From<bool> for Atom<TypeBool> {
    fn from(value: bool) -> Self {
        Atom::new(TypeBool(value))
    }
}

impl InnerTypeTrait for TypeBool {
    fn get_type() -> i16 {
        -1
    }
}

impl From<bool> for List<TypeBool> {
    fn from(v: bool) -> Self {
        List::new(vec![TypeBool::from(v)])
    }
}

impl From<Vec<bool>> for List<TypeBool> {
    fn from(values: Vec<bool>) -> Self {
        List::new(values.into_iter().map(TypeBool::from).collect())
    }
}

impl From<TypeBool> for Atom<TypeBool> {
    fn from(value: TypeBool) -> Self {
        Atom::new(value)
    }
}

impl From<TypeBool> for List<TypeBool> {
    fn from(value: TypeBool) -> Self {
        List::new(vec![value])
    }
}

impl From<Vec<TypeBool>> for List<TypeBool> {
    fn from(values: Vec<TypeBool>) -> Self {
        List::new(values)
    }
}
