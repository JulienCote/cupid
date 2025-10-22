use std::{collections::HashMap, rc::Rc};

use cupid_parser::Token;

use crate::{
    lambdas::{Lambda, LambdaRegistry},
    value::Value,
};

pub struct Context {
    pub variables: HashMap<String, Value>,
    pub lambdas: LambdaRegistry,
    // pub program: Vec<Token>,
    // pub program_counter: usize,
    pub op_stack: Vec<Rc<Lambda>>,
    pub value_stack: Vec<Value>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            variables: HashMap::new(),
            lambdas: LambdaRegistry::default(),
            op_stack: vec![],
            value_stack: vec![],
        }
    }
}

impl Context {
    pub fn load(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        self.op_stack.clear();
        self.value_stack.clear();

        for token in tokens {
            match token {
                Token::Integer(i) => self.value_stack.push(Value::Integer(i)),
                Token::Decimal(f) => self.value_stack.push(Value::Double(f)),
                Token::Operator(op) => {
                    let lambda = self
                        .lambdas
                        .lambdas
                        .get(&op.to_string())
                        .ok_or(format!("Unknown operator: {}", op))?
                        .clone();
                    self.op_stack.push(lambda);
                }
                _ => return Err(format!("Unsupported token in context load: {:?}", token)),
            }
        }

        Ok(())
    }

    pub fn execute_next(&mut self) -> Result<Value, String> {
        if self.is_complete() {
            return Err("Program has already completed".to_string());
        }

        let next_op = self.op_stack.pop().ok_or("No operation to execute")?;
        let rank = next_op.rank;
        if self.value_stack.len() < rank {
            return Err("Not enough values on the stack for operation".to_string());
        }

        let args: Vec<Value> = self
            .value_stack
            .drain(self.value_stack.len() - rank..)
            .collect();

        let result = next_op.lambda.call(&args)?;

        println!(
            "\tExecuted operation: {}, args: {:?}, result: {:?}",
            next_op.name, args, result
        );
        self.value_stack.push(result.clone());

        Ok(result)
    }

    pub fn is_complete(&self) -> bool {
        self.op_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_context() {
        let mut context = Context {
            variables: HashMap::new(),
            lambdas: LambdaRegistry::default(),
            // program: vec![],
            // program_counter: 0,
            op_stack: vec![],
            value_stack: vec![],
        };

        // Add two integers using the 'add' lambda
        context.value_stack.push(Value::Integer(3));
        context.value_stack.push(Value::Integer(5));
        context.op_stack.push(
            context
                .lambdas
                .lambdas
                .get("+")
                .expect("Add lambda should exist")
                .clone(),
        );

        let result = context.execute_next().expect("Execution should succeed");

        assert_eq!(result, Value::Integer(8));
    }
}
