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
