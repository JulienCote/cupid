use crate::lang::SuperType;

/// defines an expression but isn't evaluated.
/// This is the upmost structure representing code before being compiled into instructions.
pub struct ParseTree {
    nodes: Vec<ParseTreeNode>,
}

pub enum ParseTreeNode {
    Invokable(/* TODO */),
    Literal(SuperType),
    Variable(String),
    ParseTree(ParseTree),
}
