use crate::lang::type_system::{Atom, InnerTypeTrait, List};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct TypeByte(pub u8);

impl From<u8> for TypeByte {
    fn from(value: u8) -> Self {
        TypeByte(value)
    }
}

impl From<u8> for Atom<TypeByte> {
    fn from(value: u8) -> Self {
        Atom::new(TypeByte(value))
    }
}

impl InnerTypeTrait for TypeByte {
    fn get_type() -> i16 {
        -4
    }
}

impl From<u8> for List<TypeByte> {
    fn from(v: u8) -> Self {
        List::new(vec![TypeByte::from(v)])
    }
}

impl From<Vec<u8>> for List<TypeByte> {
    fn from(values: Vec<u8>) -> Self {
        List::new(values.into_iter().map(TypeByte::from).collect())
    }
}

impl From<TypeByte> for Atom<TypeByte> {
    fn from(value: TypeByte) -> Self {
        Atom::new(value)
    }
}

impl From<TypeByte> for List<TypeByte> {
    fn from(value: TypeByte) -> Self {
        List::new(vec![value])
    }
}

impl From<Vec<TypeByte>> for List<TypeByte> {
    fn from(values: Vec<TypeByte>) -> Self {
        List::new(values)
    }
}
