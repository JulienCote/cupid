use crate::type_system::{Atom, InnerTypeTrait, List};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TypeFloat(pub f64);

impl From<f64> for TypeFloat {
    fn from(value: f64) -> Self {
        TypeFloat(value)
    }
}

impl From<f64> for Atom<TypeFloat> {
    fn from(value: f64) -> Self {
        Atom::new(TypeFloat(value))
    }
}

impl InnerTypeTrait for TypeFloat {
    fn get_type() -> i16 {
        -9
    }
}

impl From<f64> for List<TypeFloat> {
    fn from(v: f64) -> Self {
        List::new(vec![TypeFloat::from(v)])
    }
}

impl From<Vec<f64>> for List<TypeFloat> {
    fn from(values: Vec<f64>) -> Self {
        List::new(values.into_iter().map(TypeFloat::from).collect())
    }
}

impl From<TypeFloat> for Atom<TypeFloat> {
    fn from(value: TypeFloat) -> Self {
        Atom::new(value)
    }
}

impl From<TypeFloat> for List<TypeFloat> {
    fn from(value: TypeFloat) -> Self {
        List::new(vec![value])
    }
}

impl From<Vec<TypeFloat>> for List<TypeFloat> {
    fn from(values: Vec<TypeFloat>) -> Self {
        List::new(values)
    }
}
