use crate::lang::type_system::{
    Atom, List,
    inner_types::{TypeBool, TypeByte, TypeChar, TypeFloat, TypeInt, TypeMixedList, TypeOperator},
};

#[derive(Clone, Debug, PartialEq)]
pub enum SuperType {
    Nothing,                  // TODO: Is this really needed?
    MixedList(TypeMixedList), // Mixed list is sort of special so it deserves an additional wrapper

    Bool(Atom<TypeBool>),
    Bools(List<TypeBool>),

    Byte(Atom<TypeByte>),
    Bytes(List<TypeByte>),

    Int(Atom<TypeInt>),
    Ints(List<TypeInt>),

    Float(Atom<TypeFloat>),
    Floats(List<TypeFloat>),

    Char(Atom<TypeChar>),
    Chars(List<TypeChar>),

    Table,
    Dictionary,
    Lambda,
    UnaryOperation,
    BinaryOperation(TypeOperator),
}

impl From<bool> for SuperType {
    fn from(value: bool) -> Self {
        SuperType::Bool(value.into())
    }
}

impl From<Vec<bool>> for SuperType {
    fn from(values: Vec<bool>) -> Self {
        SuperType::Bools(values.into())
    }
}

impl From<u8> for SuperType {
    fn from(value: u8) -> Self {
        SuperType::Byte(value.into())
    }
}

impl From<Vec<u8>> for SuperType {
    fn from(values: Vec<u8>) -> Self {
        SuperType::Bytes(values.into())
    }
}

impl From<i32> for SuperType {
    fn from(value: i32) -> Self {
        SuperType::Int(value.into())
    }
}

impl From<Vec<i32>> for SuperType {
    fn from(values: Vec<i32>) -> Self {
        SuperType::Ints(values.into())
    }
}

impl From<f64> for SuperType {
    fn from(value: f64) -> Self {
        SuperType::Float(value.into())
    }
}

impl From<Vec<f64>> for SuperType {
    fn from(values: Vec<f64>) -> Self {
        SuperType::Floats(values.into())
    }
}

impl From<char> for SuperType {
    fn from(value: char) -> Self {
        SuperType::Char(value.into())
    }
}

impl From<Vec<char>> for SuperType {
    fn from(values: Vec<char>) -> Self {
        SuperType::Chars(values.into())
    }
}
