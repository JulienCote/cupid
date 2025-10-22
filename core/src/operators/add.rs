use std::ops::Add;

use crate::{operators::pairwise_op, value::Value};

impl Add<Value> for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Value) -> Result<Value, String> {
        &self + &other
    }
}

impl Add<&Value> for &Value {
    type Output = Result<Value, String>;

    fn add(self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::MixedList(_), Value::MixedList(_)) => pairwise_op(&self, &other, |x, y| x + y),

            // Same type, atoms
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            (Value::Byte(a), Value::Byte(b)) => Ok(Value::Byte(a + b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a + b)),

            // Same type, lists
            (Value::Bools(_), Value::Bools(_)) => pairwise_op(&self, &other, |x, y| x + y),
            (Value::Bytes(_), Value::Bytes(_)) => pairwise_op(&self, &other, |x, y| x + y),
            (Value::Integers(_), Value::Integers(_)) => pairwise_op(&self, &other, |x, y| x + y),
            (Value::Doubles(_), Value::Doubles(_)) => pairwise_op(&self, &other, |x, y| x + y),

            // TODO: find a way not to have to write all permutations explicitly
            //       or just implement it all I guess

            // different types, atoms
            (Value::Integer(a), Value::Double(b)) => Ok(Value::Double(*a as f64 + b)),
            (Value::Double(a), Value::Integer(b)) => Ok(Value::Double(a + *b as f64)),

            // unsupported type
            (Value::Char(_), _) => Err("Cannot add to a char".to_string()),
            (_, Value::Char(_)) => Err("Cannot add to a char".to_string()),
            (Value::Chars(_), _) => Err("Cannot add to a char list".to_string()),
            (_, Value::Chars(_)) => Err("Cannot add to a char list".to_string()),
            (Value::Guid(_), _) => Err("Cannot add to a guid".to_string()),
            (_, Value::Guid(_)) => Err("Cannot add to a guid".to_string()),
            (Value::Guids(_), _) => Err("Cannot add to a guid list".to_string()),
            (_, Value::Guids(_)) => Err("Cannot add to a guid list".to_string()),

            // TODO: remove the catch-all by implementing all combinations
            (a, b) => Err(format!(
                "Unsupported types for addition: {} + {}",
                a.type_name(),
                b.type_name()
            )),
        }
    }
}

#[cfg(test)]
mod tests_success {
    use super::*;

    #[test]
    fn add_integers() {
        let a = Value::Integer(5);
        let b = Value::Integer(10);
        let result = a + b;
        assert_eq!(result, Ok(Value::Integer(15)));
    }

    #[test]
    fn add_doubles() {
        let a = Value::Double(2.5);
        let b = Value::Double(3.5);
        let result = a + b;
        assert_eq!(result, Ok(Value::Double(6.0)));
    }

    #[test]
    fn add_integer_and_double() {
        let a = Value::Integer(4);
        let b = Value::Double(2.5);
        let result = a + b;
        assert_eq!(result, Ok(Value::Double(6.5)));
    }

    #[test]
    fn add_mixed_lists() {
        let a = Value::MixedList(vec![Value::Integer(1), Value::Double(2.0)]);
        let b = Value::MixedList(vec![Value::Integer(3), Value::Double(4.0)]);
        let result = a + b;
        assert_eq!(
            result,
            Ok(Value::MixedList(vec![
                Value::Integer(4),
                Value::Double(6.0)
            ]))
        );
    }

    #[test]
    fn add_atom_to_list() {
        let a = Value::Integer(5);
        let b = Value::MixedList(vec![Value::Integer(1), Value::Double(2.0)]);
        let expected = Value::MixedList(vec![Value::Integer(6), Value::Double(7.0)]);
        let result = a + b;
        assert_eq!(result, Ok(expected));
    }
}

#[cfg(test)]
mod tests_failure {
    use super::*;

    #[test]
    fn add_char_and_integer() {
        let a = Value::Char('x');
        let b = Value::Integer(10);
        let result = a + b;
        assert!(result.is_err());
    }

    #[test]
    fn add_char_list_and_integer() {
        let a = Value::Chars(vec!['a', 'b', 'c']);
        let b = Value::Integer(5);
        let result = a + b;
        assert!(result.is_err());
    }

    #[test]
    fn add_lists_of_different_ranks() {
        let a = Value::Integers(vec![1, 2]);
        let b = Value::Integers(vec![3, 4, 5]);

        let result = a + b;
        assert!(result.is_err());
    }
}
