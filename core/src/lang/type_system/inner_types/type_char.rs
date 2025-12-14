use crate::lang::type_system::{Atom, InnerTypeTrait, List};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct TypeChar(pub char);

impl From<char> for TypeChar {
    fn from(value: char) -> Self {
        TypeChar(value)
    }
}

impl From<char> for Atom<TypeChar> {
    fn from(value: char) -> Self {
        Atom::new(TypeChar(value))
    }
}

impl InnerTypeTrait for TypeChar {
    fn get_type() -> i16 {
        -10
    }
}

impl From<char> for List<TypeChar> {
    fn from(v: char) -> Self {
        List::new(vec![TypeChar::from(v)])
    }
}

impl From<Vec<char>> for List<TypeChar> {
    fn from(values: Vec<char>) -> Self {
        List::new(values.into_iter().map(TypeChar::from).collect())
    }
}

impl From<TypeChar> for Atom<TypeChar> {
    fn from(value: TypeChar) -> Self {
        Atom::new(value)
    }
}

impl From<TypeChar> for List<TypeChar> {
    fn from(value: TypeChar) -> Self {
        List::new(vec![value])
    }
}

impl From<Vec<TypeChar>> for List<TypeChar> {
    fn from(values: Vec<TypeChar>) -> Self {
        List::new(values)
    }
}
