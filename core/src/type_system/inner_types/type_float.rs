use crate::type_system::{
    NumericOps, Promote, Type,
    inner_types::{TypeFloats, TypeInt, TypeInts},
};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub struct TypeFloat(pub f32);

impl From<f32> for TypeFloat {
    fn from(value: f32) -> Self {
        TypeFloat(value)
    }
}

impl Type for TypeFloat {
    fn name(&self) -> &str {
        "Float"
    }

    fn type_id(&self) -> char {
        'f'
    }

    fn attributes(&self) -> u8 {
        0
    }

    fn size(&self) -> usize {
        1
    }
}

// Implement NumericOps for TypeFloat (opt-in to numeric operations)
impl NumericOps for TypeFloat {
    #[inline]
    fn add(self, rhs: Self) -> Self {
        TypeFloat(self.0 + rhs.0)
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        TypeFloat(self.0 - rhs.0)
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        TypeFloat(self.0 * rhs.0)
    }

    #[inline]
    fn div(self, rhs: Self) -> Self {
        TypeFloat(self.0 / rhs.0)
    }
}

// Std ops for convenience
impl Add for TypeFloat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        NumericOps::add(self, rhs)
    }
}

impl Sub for TypeFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        NumericOps::sub(self, rhs)
    }
}

impl Mul for TypeFloat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        NumericOps::mul(self, rhs)
    }
}

impl Div for TypeFloat {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        NumericOps::div(self, rhs)
    }
}

// TypeFloat promotes TypeInt to TypeFloat (reverse of TypeInt + TypeFloat)
impl Promote<TypeInt> for TypeFloat {
    type Output = TypeFloat;

    #[inline]
    fn promote_pair(self, rhs: TypeInt) -> (TypeFloat, TypeFloat) {
        (self, TypeFloat(rhs.0 as f32))
    }
}

// Reverse broadcasting: float scalar operates with int vector (promotes ints to floats)
impl crate::type_system::ReverseBroadcastOps<TypeInts> for TypeFloat {
    type Output = TypeFloats;

    #[inline]
    fn rbroadcast_add(self, vector: TypeInts) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 + x as f32).collect())
    }

    #[inline]
    fn rbroadcast_sub(self, vector: TypeInts) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 - x as f32).collect())
    }

    #[inline]
    fn rbroadcast_mul(self, vector: TypeInts) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 * x as f32).collect())
    }

    #[inline]
    fn rbroadcast_div(self, vector: TypeInts) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 / x as f32).collect())
    }
}

// No promotion needed when both are TypeFloat
impl Promote<TypeFloat> for TypeFloat {
    type Output = TypeFloat;

    #[inline]
    fn promote_pair(self, rhs: TypeFloat) -> (TypeFloat, TypeFloat) {
        (self, rhs)
    }
}
