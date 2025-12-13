use std::rc::Rc;

use crate::type_system::SuperType;

#[derive(Debug, Clone)]
pub enum Instruction {
    // A statement is a sequence of instructions that runs in the current context
    BeginStatementSilent(Vec<Instruction>),
    BeginStatement(Vec<Instruction>),

    // Ending A non-silent statement implicitely invokes Return (which returns the last rvalue)
    EndStatement,

    // A Lambda creates a new context to run with. That context becomes a child of the current context.
    Lambda {
        args_push: Vec<Instruction>,
        body: Vec<Instruction>,
    },

    // A Return ends the current context and returns the last rvalue pushed onto the rvalue stack.
    Return,

    SetVariableNew {
        name: String,
        value: SuperType,
    },
    SetVariableRef {
        name: String,
        value: Rc<SuperType>,
    },
    GetVariable {
        name: String,
    },
    SetGlobalVariableNew {
        name: String,
        value: SuperType,
    },
    SetGlobalVariableRef {
        name: String,
        value: Rc<SuperType>,
    },
    GetGlobalVariable {
        name: String,
    },

    UnaryOperation {
        op: String,
        value: SuperType,
    },
    BinaryOperation {
        op: String,
        left: SuperType,
        right: SuperType,
    },
    Literal(SuperType),
}
