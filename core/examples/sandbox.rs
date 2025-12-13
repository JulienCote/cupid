use std::vec;

use cupid_core::{
    sandbox::{execution_context::ExecutionContext, instruction::Instruction},
    type_system::SuperType,
};

fn main() {
    let mut global_context = ExecutionContext::default();

    // Example: Create a simple statement that sets a variable and retrieves it
    // It's a stack, so instructions are pushed in reverse order
    global_context.push_statement_silent(vec![
        Instruction::GetVariable {
            name: "a".to_string(),
        },
        Instruction::SetVariableNew {
            name: "a".to_string(),
            value: SuperType::Int(10.into()),
        },
        Instruction::Lambda {
            args_push: vec![],
            body: vec![],
        },
    ]);

    match global_context.unwind() {
        Ok(value) => println!("Execution succeeded with value: {:?}", value),
        Err(e) => println!("Execution failed with error: {}", e),
    }
}
