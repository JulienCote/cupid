use crate::type_system::{
    BroadcastOps, NumericOps, Promote, ReverseBroadcastOps, Type,
    inner_types::{TypeFloat, TypeFloats, TypeInt, TypeInts},
    super_type::SuperType,
};

/// Operators that can be applied to types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Trait to calculate the result of an operator on two types
/// This unifies NumericOps, Promote, and BroadcastOps into a single interface
pub trait Calculate<Rhs> {
    type Output: Type;
    fn calculate(op: Operator, lhs: Self, rhs: Rhs) -> Self::Output;
}

// Macro to implement Calculate for types that support NumericOps (Same Type)
macro_rules! impl_calc_numeric {
    ($L:ty, $R:ty) => {
        impl Calculate<$R> for $L {
            type Output = $L;
            #[inline]
            fn calculate(op: Operator, lhs: $L, rhs: $R) -> Self::Output {
                match op {
                    Operator::Add => lhs.add(rhs),
                    Operator::Subtract => lhs.sub(rhs),
                    Operator::Multiply => lhs.mul(rhs),
                    Operator::Divide => lhs.div(rhs),
                }
            }
        }
    };
}

// Macro to implement Calculate via Promotion (Scalar <-> Scalar)
macro_rules! impl_calc_promote {
    ($L:ty, $R:ty) => {
        impl Calculate<$R> for $L {
            type Output = <$L as Promote<$R>>::Output;
            #[inline]
            fn calculate(op: Operator, lhs: $L, rhs: $R) -> Self::Output {
                let (a, b) = lhs.promote_pair(rhs);
                match op {
                    Operator::Add => a.add(b),
                    Operator::Subtract => a.sub(b),
                    Operator::Multiply => a.mul(b),
                    Operator::Divide => a.div(b),
                }
            }
        }
    };
}

// Macro to implement Calculate via Broadcasting (Vector op Scalar)
macro_rules! impl_calc_broadcast {
    ($L:ty, $R:ty) => {
        impl Calculate<$R> for $L {
            type Output = <$L as BroadcastOps<$R>>::Output;
            #[inline]
            fn calculate(op: Operator, lhs: $L, rhs: $R) -> Self::Output {
                match op {
                    Operator::Add => lhs.broadcast_add(rhs),
                    Operator::Subtract => lhs.broadcast_sub(rhs),
                    Operator::Multiply => lhs.broadcast_mul(rhs),
                    Operator::Divide => lhs.broadcast_div(rhs),
                }
            }
        }
    };
}

// Macro to implement Calculate via Reverse Broadcasting (Scalar op Vector)
macro_rules! impl_calc_rbroadcast {
    ($L:ty, $R:ty) => {
        impl Calculate<$R> for $L {
            type Output = <$L as ReverseBroadcastOps<$R>>::Output;
            #[inline]
            fn calculate(op: Operator, lhs: $L, rhs: $R) -> Self::Output {
                match op {
                    Operator::Add => lhs.rbroadcast_add(rhs),
                    Operator::Subtract => lhs.rbroadcast_sub(rhs),
                    Operator::Multiply => lhs.rbroadcast_mul(rhs),
                    Operator::Divide => lhs.rbroadcast_div(rhs),
                }
            }
        }
    };
}

// --- Implementations ---

// 1. Same Types (NumericOps)
impl_calc_numeric!(TypeInt, TypeInt);
impl_calc_numeric!(TypeFloat, TypeFloat);
impl_calc_numeric!(TypeInts, TypeInts);
impl_calc_numeric!(TypeFloats, TypeFloats);

// 2. Promotion (Scalar <-> Scalar)
impl_calc_promote!(TypeInt, TypeFloat);
impl_calc_promote!(TypeFloat, TypeInt);

// 3. Promotion (Vector <-> Vector)
impl_calc_promote!(TypeInts, TypeFloats);
impl_calc_promote!(TypeFloats, TypeInts);

// 4. Broadcasting (Vector op Scalar)
impl_calc_broadcast!(TypeInts, TypeInt);
impl_calc_broadcast!(TypeInts, TypeFloat);
impl_calc_broadcast!(TypeFloats, TypeInt);
impl_calc_broadcast!(TypeFloats, TypeFloat);

// 5. Reverse Broadcasting (Scalar op Vector)
impl_calc_rbroadcast!(TypeInt, TypeInts);
impl_calc_rbroadcast!(TypeInt, TypeFloats);
impl_calc_rbroadcast!(TypeFloat, TypeInts);
impl_calc_rbroadcast!(TypeFloat, TypeFloats);

impl Operator {
    /// Apply an operator to two values
    /// Automatically selects the best strategy:
    /// - NumericOps for same types
    /// - Promotion for compatible scalars (Int -> Float)
    /// - Efficient Broadcasting for Scalar <-> Vector (NO allocation)
    #[inline]
    pub fn apply<L, R>(
        &self,
        lhs: SuperType<L>,
        rhs: SuperType<R>,
    ) -> SuperType<<L as Calculate<R>>::Output>
    where
        L: Type + Calculate<R>,
        R: Type,
    {
        SuperType(Calculate::calculate(*self, lhs.0, rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_add_same_type() {
        let lhs = SuperType(TypeInt(10));
        let rhs = SuperType(TypeInt(32));
        let result = Operator::Add.apply(lhs, rhs);
        assert_eq!(result.0.size(), 1);
        assert_eq!(result.0.0, 42);
    }

    #[test]
    fn operator_subtract() {
        let lhs = SuperType(TypeInt(50));
        let rhs = SuperType(TypeInt(8));
        let result = Operator::Subtract.apply(lhs, rhs);
        assert_eq!(result.0.0, 42);
    }

    #[test]
    fn operator_multiply() {
        let lhs = SuperType(TypeInt(6));
        let rhs = SuperType(TypeInt(7));
        let result = Operator::Multiply.apply(lhs, rhs);
        assert_eq!(result.0.0, 42);
    }

    #[test]
    fn operator_divide() {
        let lhs = SuperType(TypeInt(84));
        let rhs = SuperType(TypeInt(2));
        let result = Operator::Divide.apply(lhs, rhs);
        assert_eq!(result.0.0, 42);
    }

    #[test]
    fn operator_with_promotion_int_float() {
        let lhs = SuperType(TypeInt(10));
        let rhs = SuperType(TypeFloat(32.5));
        let result = Operator::Add.apply(lhs, rhs);
        assert_eq!(result.0.0, 42.5);
    }

    #[test]
    fn operator_with_promotion_float_int() {
        let lhs = SuperType(TypeFloat(10.5));
        let rhs = SuperType(TypeInt(32));
        let result = Operator::Add.apply(lhs, rhs);
        assert_eq!(result.0.0, 42.5);
    }

    #[test]
    fn operator_atom_broadcast_to_list() {
        // Test Int + Ints broadcasts the scalar efficiently (no vector allocation)
        let scalar = SuperType(TypeInt(10));
        let vector = SuperType(TypeInts(vec![1, 2, 3]));
        let result = Operator::Add.apply(scalar, vector);
        assert_eq!(result.0.0, vec![11, 12, 13]);
    }

    #[test]
    fn operator_list_with_atom() {
        // Test Ints + Int broadcasts the scalar efficiently
        let vector = SuperType(TypeInts(vec![5, 10, 15]));
        let scalar = SuperType(TypeInt(5));
        let result = Operator::Multiply.apply(vector, scalar);
        assert_eq!(result.0.0, vec![25, 50, 75]);
    }

    #[test]
    fn operator_float_broadcast_to_ints() {
        // Test Float + Ints promotes to TypeFloats and broadcasts efficiently
        let scalar = SuperType(TypeFloat(2.5));
        let vector = SuperType(TypeInts(vec![10, 20, 30]));
        let result = Operator::Multiply.apply(scalar, vector);
        assert_eq!(result.0.0, vec![25.0, 50.0, 75.0]);
    }
}
