pub mod add;

use crate::value::Value;

pub fn check_same_rank(a: &Value, b: &Value) -> bool {
    a.rank() == b.rank()
}

pub fn pairwise_op<F>(a: &Value, b: &Value, op: F) -> Result<Value, String>
where
    F: Fn(&Value, &Value) -> Result<Value, String>,
{
    if !check_same_rank(a, b) {
        return Err("Values must have the same rank for pairwise operations".to_string());
    }

    match (a, b) {
        (Value::MixedList(va), Value::MixedList(vb)) => {
            let result: Result<Vec<Value>, String> =
                va.iter().zip(vb.iter()).map(|(x, y)| op(x, y)).collect();
            Ok(Value::MixedList(result?))
        }
        _ => op(a, b),
    }
}

// pub fn atom_on_list_op<F>(a: &Value, b: &Value, op: F) -> Result<Value, String>
// where
//     F: Fn(&Value, &Value) -> Result<Value, String>,
// {
//     let (atom, list) = match (a.rank(), b.rank()) {
//         (1, r) if r > 1 => (a, b),
//         (r, 1) if r > 1 => (b, a),
//         _ => {
//             return Err(
//                 "One argument must be an atom (rank 1), and the other must be a list (rank > 1), for atom-on-list operations".to_string(),
//             );
//         }
//     };

//     // list.into_iter().map()
// }
