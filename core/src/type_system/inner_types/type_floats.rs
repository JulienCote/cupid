use crate::type_system::{
    NumericOps, Promote, Type,
    inner_types::{TypeFloat, TypeInt, TypeInts},
};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub struct TypeFloats(pub Vec<f32>);

impl From<Vec<f32>> for TypeFloats {
    fn from(value: Vec<f32>) -> Self {
        TypeFloats(value)
    }
}

impl Type for TypeFloats {
    fn name(&self) -> &str {
        "Floats"
    }

    fn type_id(&self) -> char {
        'F'
    }

    fn attributes(&self) -> u8 {
        0
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

// Implement NumericOps for TypeFloats (element-wise operations)
impl NumericOps for TypeFloats {
    #[inline]
    fn add(self, rhs: Self) -> Self {
        TypeFloats(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        TypeFloats(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        TypeFloats(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a * b)
                .collect(),
        )
    }

    #[inline]
    fn div(self, rhs: Self) -> Self {
        TypeFloats(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a / b)
                .collect(),
        )
    }
}

// Std ops for convenience
impl Add for TypeFloats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        NumericOps::add(self, rhs)
    }
}

impl Sub for TypeFloats {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        NumericOps::sub(self, rhs)
    }
}

impl Mul for TypeFloats {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        NumericOps::mul(self, rhs)
    }
}

impl Div for TypeFloats {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        NumericOps::div(self, rhs)
    }
}

// Efficient broadcasting: vector operates with float scalar
impl crate::type_system::BroadcastOps<TypeFloat> for TypeFloats {
    type Output = TypeFloats;

    #[inline]
    fn broadcast_add(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x + scalar.0).collect())
    }

    #[inline]
    fn broadcast_sub(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x - scalar.0).collect())
    }

    #[inline]
    fn broadcast_mul(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x * scalar.0).collect())
    }

    #[inline]
    fn broadcast_div(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x / scalar.0).collect())
    }
}

// Efficient broadcasting: vector operates with int scalar (promoted to float)
impl crate::type_system::BroadcastOps<TypeInt> for TypeFloats {
    type Output = TypeFloats;

    #[inline]
    fn broadcast_add(self, scalar: TypeInt) -> Self::Output {
        let s = scalar.0 as f32;
        TypeFloats(self.0.into_iter().map(|x| x + s).collect())
    }

    #[inline]
    fn broadcast_sub(self, scalar: TypeInt) -> Self::Output {
        let s = scalar.0 as f32;
        TypeFloats(self.0.into_iter().map(|x| x - s).collect())
    }

    #[inline]
    fn broadcast_mul(self, scalar: TypeInt) -> Self::Output {
        let s = scalar.0 as f32;
        TypeFloats(self.0.into_iter().map(|x| x * s).collect())
    }

    #[inline]
    fn broadcast_div(self, scalar: TypeInt) -> Self::Output {
        let s = scalar.0 as f32;
        TypeFloats(self.0.into_iter().map(|x| x / s).collect())
    }
}

// Reverse broadcasting: float scalar operates with vector
impl crate::type_system::ReverseBroadcastOps<TypeFloats> for TypeFloat {
    type Output = TypeFloats;

    #[inline]
    fn rbroadcast_add(self, vector: TypeFloats) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 + x).collect())
    }

    #[inline]
    fn rbroadcast_sub(self, vector: TypeFloats) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 - x).collect())
    }

    #[inline]
    fn rbroadcast_mul(self, vector: TypeFloats) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 * x).collect())
    }

    #[inline]
    fn rbroadcast_div(self, vector: TypeFloats) -> Self::Output {
        TypeFloats(vector.0.into_iter().map(|x| self.0 / x).collect())
    }
}

// Promote TypeInts to TypeFloats (element-wise promotion, no broadcasting)
impl Promote<TypeInts> for TypeFloats {
    type Output = TypeFloats;

    #[inline]
    fn promote_pair(self, rhs: TypeInts) -> (TypeFloats, TypeFloats) {
        let promoted_rhs: Vec<f32> = rhs.0.iter().map(|&x| x as f32).collect();
        (self, TypeFloats(promoted_rhs))
    }
}

// No promotion needed when both are TypeFloats
impl Promote<TypeFloats> for TypeFloats {
    type Output = TypeFloats;

    #[inline]
    fn promote_pair(self, rhs: TypeFloats) -> (TypeFloats, TypeFloats) {
        (self, rhs)
    }
}
