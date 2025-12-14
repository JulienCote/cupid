use crate::lang::type_system::{Atom, InnerTypeTrait, List};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct TypeInt(pub i32);

impl From<i32> for TypeInt {
    fn from(value: i32) -> Self {
        TypeInt(value)
    }
}

impl From<i32> for Atom<TypeInt> {
    fn from(value: i32) -> Self {
        Atom::new(TypeInt(value))
    }
}

impl InnerTypeTrait for TypeInt {
    fn get_type() -> i16 {
        -6
    }
}

impl From<i32> for List<TypeInt> {
    fn from(v: i32) -> Self {
        List::new(vec![TypeInt::from(v)])
    }
}

impl From<Vec<i32>> for List<TypeInt> {
    fn from(values: Vec<i32>) -> Self {
        List::new(values.into_iter().map(TypeInt::from).collect())
    }
}

impl From<TypeInt> for Atom<TypeInt> {
    fn from(value: TypeInt) -> Self {
        Atom::new(value)
    }
}

impl From<TypeInt> for List<TypeInt> {
    fn from(value: TypeInt) -> Self {
        List::new(vec![value])
    }
}

impl From<Vec<TypeInt>> for List<TypeInt> {
    fn from(values: Vec<TypeInt>) -> Self {
        List::new(values)
    }
}
