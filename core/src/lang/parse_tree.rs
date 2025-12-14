use crate::lang::SuperType;

/// defines an expression but isn't evaluated.
/// This is the upmost structure representing code before being compiled into instructions.
pub struct ParseTree {
    //TODO: invokable: Invokable,
    arguments: Vec<ParseTreeNode>,
}

pub enum ParseTreeNode {
    Literal(SuperType),   // just a value
    Variable(String),     // variable name, needs to be resolved
    ParseTree(ParseTree), // nested parse tree, e.g. for function calls
}
