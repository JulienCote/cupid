use crate::Error;

// Every operator takes 2 SuperType and returns a SuperType
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // %

    // Equality operators
    Equals,    // =
    NotEquals, // <>
    Match,     // ~

    // Ordinal Comparison operators
    LessThan,    // <
    UpTo,        // <=
    AtLeast,     // >=
    GreaterThan, // >

    // List operators
    Join, // ,
    Fill, // ^
    Take, // #
    Cut,  // _

    // Assignement operators
    Assign, // :

    // Logical operators
    And, // &
    Or,  // |
}

pub trait OperatorAdd<Tin, Tout> {
    fn add(_lhs: Tin, _rhs: Tin) -> Result<Tout, Error> {
        Err(Error::Type)
    }
}

pub trait OperatorSubtract<Tin, Tout> {
    fn sub(_lhs: Tin, _rhs: Tin) -> Result<Tout, Error> {
        Err(Error::Type)
    }
}

pub trait OperatorMultiply<Tin, Tout> {
    fn mul(_lhs: Tin, _rhs: Tin) -> Result<Tout, Error> {
        Err(Error::Type)
    }
}

pub trait OperatorDivide<Tin, Tout> {
    fn div(_lhs: Tin, _rhs: Tin) -> Result<Tout, Error> {
        Err(Error::Type)
    }
}

pub trait OperatorEquals<Tin> {
    fn equals(lhs: Tin, rhs: Tin) -> bool;
}
