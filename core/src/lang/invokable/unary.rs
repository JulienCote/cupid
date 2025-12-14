// Every unary operator takes 1 SuperType and returns a SuperType
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Unary {
    // List manipulation
    Count,
    Cut,
    Enlist,
    First,
    In,
    Last,
    Raze,
    Reverse,
    Til,

    // IO
    Get,
    Set,

    // Logical
    All,
    Any,
    Not,

    // Meta
    Null,
    Attributes,

    // Assignement
    AssignThrough, // .: @: $: !: ?: +: -: *: %: =: ~: <: >: |: &: #: _: ^: ,:
}
