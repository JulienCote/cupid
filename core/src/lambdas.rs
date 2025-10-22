use std::{collections::HashMap, rc::Rc};

use crate::{context::Context, value::Value};

pub trait Callable {
    fn call(&self, args: &[Value]) -> Result<Value, String>;
}

pub trait CallableMut {
    fn call_mut(&self, context: &mut Context, args: &[Value]) -> Result<Value, String>;
}

impl<F> Callable for F
where
    F: Fn(&[Value]) -> Result<Value, String>,
{
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        self(args)
    }
}

impl<F> CallableMut for F
where
    F: Fn(&mut Context, &[Value]) -> Result<Value, String>,
{
    fn call_mut(&self, context: &mut Context, args: &[Value]) -> Result<Value, String> {
        self(context, args)
    }
}

pub enum LambdaType {
    Regular(Box<dyn Callable>),
    Mutating(Box<dyn CallableMut>),
}

pub struct Lambda {
    pub name: String,
    pub rank: usize,
    // pub args: Vec<value::Value>,
    pub lambda: LambdaType,
}

impl Callable for Lambda {
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match &self.lambda {
            LambdaType::Regular(f) => f.call(args),
            LambdaType::Mutating(_) => Err("This lambda requires mutable context".to_string()),
        }
    }
}

impl CallableMut for Lambda {
    fn call_mut(&self, context: &mut Context, args: &[Value]) -> Result<Value, String> {
        match &self.lambda {
            LambdaType::Regular(_) => {
                Err("This lambda does not support mutable context".to_string())
            }
            LambdaType::Mutating(f) => f.call_mut(context, args),
        }
    }
}

pub struct LambdaRegistry {
    pub lambdas: HashMap<String, Rc<Lambda>>,
}

impl LambdaRegistry {
    pub fn register_lambda<F>(&mut self, name: &str, rank: usize, operation: F)
    where
        F: 'static + Callable,
    {
        let lambda = Lambda {
            name: name.to_string(),
            rank,
            // args: Vec::new(),
            lambda: LambdaType::Regular(Box::new(operation)),
        };
        self.lambdas.insert(name.to_string(), Rc::new(lambda));
    }

    pub fn register_lambda_mut<F>(&mut self, name: &str, rank: usize, operation: F)
    where
        F: 'static + CallableMut,
    {
        let lambda = Lambda {
            name: name.to_string(),
            rank,
            lambda: LambdaType::Mutating(Box::new(operation)),
        };
        self.lambdas.insert(name.to_string(), Rc::new(lambda));
    }
}

impl Default for LambdaRegistry {
    fn default() -> Self {
        let mut lambdas = HashMap::new();

        // add an addition lambda
        lambdas.insert(
            "+".to_string(),
            Rc::new(Lambda {
                name: "add".to_string(),
                rank: 2,
                lambda: LambdaType::Regular(Box::new(|args: &[Value]| {
                    if args.len() != 2 {
                        return Err("Addition requires exactly two arguments".to_string());
                    }
                    return &args[0] + &args[1];
                })),
            }),
        );

        // add an assignement lambda
        lambdas.insert(
            ":".to_string(),
            Rc::new(Lambda {
                name: "assign".to_string(),
                rank: 2,
                lambda: LambdaType::Mutating(Box::new(|context: &mut Context, args: &[Value]| {
                    if args.len() != 2 {
                        return Err("Assignment requires exactly two values".to_string());
                    };

                    // TODO: actually manage identifiers as symbols, not strings
                    let var_name = match &args[0] {
                        Value::Chars(c) => c.clone().iter().collect(),
                        _ => {
                            return Err(
                                "First argument to assignment must be a variable name (Char)"
                                    .to_string(),
                            );
                        }
                    };
                    let value = &args[1];
                    context.variables.insert(var_name, value.clone());
                    Ok(value.clone())
                })),
            }),
        );
        Self { lambdas }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambda_add() {
        let registry = LambdaRegistry::default();

        // Test calling the "add" lambda
        let result = registry
            .lambdas
            .get("+")
            .unwrap()
            .call(&[Value::Integer(2), Value::Integer(3)]);
        assert_eq!(result, Ok(Value::Integer(5)));
    }
}
