use std::{cell::RefCell, rc::Rc};

use crate::{
    Error,
    core::{instruction::Instruction, variable::Variables},
    lang::SuperType,
};

#[derive(Clone, Debug, Default)]
pub struct ExecutionContext {
    parent: Option<Rc<RefCell<ExecutionContext>>>, // Parent context for variable scoping

    variables: Variables,           // Local variables
    instructions: Vec<Instruction>, // Stack of instructions - what the program does
    rvalues: Vec<EphemeralValue>,   // Stack of rvalues - temporary values for calculations
}

#[derive(Clone, Debug, PartialEq)]
pub enum EphemeralValue {
    Owned(SuperType),
    Ref(Rc<SuperType>),
}

impl From<SuperType> for EphemeralValue {
    fn from(value: SuperType) -> Self {
        EphemeralValue::Owned(value)
    }
}

impl From<Rc<SuperType>> for EphemeralValue {
    fn from(value: Rc<SuperType>) -> Self {
        EphemeralValue::Ref(value)
    }
}

impl ExecutionContext {
    pub fn push_statement(&mut self, instructions: Vec<Instruction>) {
        self.instructions.push(Instruction::EndStatement);
        self.instructions
            .push(Instruction::BeginStatement(instructions));
    }

    pub fn push_statement_silent(&mut self, instructions: Vec<Instruction>) {
        self.instructions.push(Instruction::EndStatement);
        self.instructions
            .push(Instruction::BeginStatementSilent(instructions));
    }

    pub fn unwind(&mut self) -> Result<EphemeralValue, Error> {
        let mut is_statement_silent = false;
        while let Some(instr) = self.instructions.pop() {
            // TODO: integrate in tracing
            // println!("Executing instruction: {:?}", instr);
            // println!("Current state: {self:#?}");
            match instr {
                Instruction::BeginStatementSilent(instructions) => {
                    is_statement_silent = true;
                    self.instructions.extend(instructions.into_iter());
                }
                Instruction::BeginStatement(instructions) => {
                    is_statement_silent = false;
                    self.instructions.extend(instructions.into_iter());
                }
                Instruction::EndStatement => {
                    match is_statement_silent {
                        true => {
                            // pop it either way
                            self.rvalues.pop();
                            is_statement_silent = false;
                        }
                        false => return self.do_return(false),
                    }
                }
                Instruction::Lambda { args_push, body } => {
                    // create a new context with the current as parent
                    // run/unwind the context

                    // grab the returned value and push it onto the rvalue stack
                    todo!("Lambda execution not implemented yet");
                }
                Instruction::Return => return self.do_return(true),
                Instruction::SetVariableNew { name, value } => {
                    let var = self.variables.set_new(name, value);
                    self.rvalues.push(var.into());
                }
                Instruction::SetVariableRef { name, value } => {
                    let var = self.variables.set_ref(name, value);
                    self.rvalues.push(var.into());
                }
                Instruction::GetVariable { name } => {
                    // Get variable from the current context
                    let var = self.variables.get(&name).ok_or(Error::Name(name))?;
                    self.rvalues.push(var.into());
                }
                Instruction::SetGlobalVariableNew { name, value } => {
                    let var = self.set_global_new(name, value);
                    self.rvalues.push(var.into());
                }
                Instruction::SetGlobalVariableRef { name, value } => {
                    let var = self.set_global_ref(name, value);
                    self.rvalues.push(var.into());
                }
                Instruction::GetGlobalVariable { name } => {
                    let var = self.get_global(&name).ok_or(Error::Name(name))?;
                    self.rvalues.push(var.into());
                }

                Instruction::UnaryOperation { op: _, value: _ } => {}
                Instruction::BinaryOperation {
                    op: _,
                    left: _,
                    right: _,
                } => {}

                Instruction::Literal(value) => {
                    self.rvalues.push(value.into());
                    continue;
                }
            };
        }

        Ok(self
            .rvalues
            .pop()
            .unwrap_or(EphemeralValue::Owned(SuperType::Nothing)))
    }

    fn set_global_new(&mut self, name: String, value: SuperType) -> Rc<SuperType> {
        match &self.parent {
            Some(parent_ctx) => parent_ctx.borrow_mut().set_global_new(name, value),
            None => self.variables.set_new(name, value),
        }
    }

    fn set_global_ref(&mut self, name: String, value: Rc<SuperType>) -> Rc<SuperType> {
        match &self.parent {
            Some(parent_ctx) => parent_ctx.borrow_mut().set_global_ref(name, value),
            None => self.variables.set_ref(name, value),
        }
    }

    fn get_global(&self, name: &str) -> Option<Rc<SuperType>> {
        match &self.parent {
            Some(parent_ctx) => parent_ctx.borrow().get_global(name),
            None => self.variables.get(name),
        }
    }

    fn is_global_context(&self) -> bool {
        self.parent.is_none()
    }

    // exlicit indicates if the return was explicit (via Return instruction) or implicit (via EndStatement)
    fn do_return(&mut self, explicit: bool) -> Result<EphemeralValue, Error> {
        // cannot return from the global context
        if self.is_global_context() && explicit {
            return Err(Error::NotYetImplemented);
        }

        let val = self.rvalues.pop().ok_or_else(|| {
            Error::MalformedProgram("Returning but no values left to return.".to_owned())
        })?;

        // clear the context so we can move local refs
        self.clear();

        // attempt to unwrap Rc if possible
        match val {
            EphemeralValue::Ref(rc) => {
                // println!("Returning value: {:?}", val);
                println!("Strong reference count: {}", Rc::strong_count(&rc));
                match Rc::try_unwrap(rc) {
                    Ok(inner) => Ok(EphemeralValue::Owned(inner)),
                    Err(rc) => Ok(EphemeralValue::Ref(rc)),
                }
            }
            v => Ok(v),
        }
    }

    /// Clears the execution context of all instructions and rvalues.
    /// If not in the global context, also clears local variables.
    fn clear(&mut self) {
        if !self.is_global_context() {
            self.variables.clear();
            self.parent = None;
        }
        self.rvalues.clear();
        self.instructions.clear();
    }
}

impl Drop for ExecutionContext {
    fn drop(&mut self) {
        self.clear();
        println!("Dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Equivalent program:
    /// > :42
    /// 'nyi
    #[test]
    fn test_execution_context_return_from_global() {
        let mut context = ExecutionContext::default();
        context.instructions.push(Instruction::Return);
        context
            .instructions
            .push(Instruction::Literal(SuperType::Int(42.into())));

        let outcome = context.unwind();

        assert!(outcome.is_err());
        assert_eq!(outcome.unwrap_err(), Error::NotYetImplemented);
    }

    /// Program would be equivalent to:
    /// > x: 42;
    /// > x
    /// 42
    #[test]
    fn test_execution_context_unwind_simplest_function() {
        let mut context = ExecutionContext::default();

        // access x then return
        context.push_statement(vec![Instruction::GetVariable {
            name: "x".to_string(),
        }]);

        // Set the variable x to 42
        context.push_statement_silent(vec![Instruction::SetVariableNew {
            name: "x".to_string(),
            value: SuperType::Int(42.into()),
        }]);

        let outcome = context.unwind();

        assert!(outcome.is_ok());
        assert_eq!(
            outcome.unwrap(),
            EphemeralValue::Ref(SuperType::Int(42.into()).into())
        );
    }
}
