use std::{collections::HashMap, rc::Rc};

use crate::value::Value;

pub trait Callable {
    fn call(&self, args: &[Value]) -> Result<Value, String>;
    // fn rank(&self) -> usize;
}

impl<F> Callable for F
where
    F: Fn(&[Value]) -> Result<Value, String>,
{
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        self(args)
    }

    // fn rank(&self) -> usize {
    //     unreachable!("Callable rank should be provided by the Lambda struct")
    // }
}

pub struct Lambda {
    pub name: String,
    pub rank: usize,
    // pub args: Vec<value::Value>,
    pub lambda: Box<dyn Callable>,
}

impl Callable for Lambda {
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        (self.lambda).call(args)
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
            lambda: Box::new(operation),
        };
        self.lambdas.insert(name.to_string(), Rc::new(lambda));
    }
}

impl Default for LambdaRegistry {
    fn default() -> Self {
        let mut lambdas = HashMap::new();

        // add an addition lambdas
        lambdas.insert(
            "+".to_string(),
            Rc::new(Lambda {
                name: "add".to_string(),
                rank: 2,
                lambda: Box::new(|args: &[Value]| {
                    if args.len() != 2 {
                        return Err("Addition requires exactly two arguments".to_string());
                    }
                    return &args[0] + &args[1];
                }),
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
