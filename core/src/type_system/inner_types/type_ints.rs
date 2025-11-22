use crate::type_system::inner_types::{TypeFloat, TypeFloats, TypeInt};
use crate::type_system::{NumericOps, Promote, Type};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub struct TypeInts(pub Vec<i32>);

impl From<Vec<i32>> for TypeInts {
    fn from(vec: Vec<i32>) -> Self {
        TypeInts(vec)
    }
}

impl Type for TypeInts {
    fn name(&self) -> &str {
        "Ints"
    }

    fn type_id(&self) -> char {
        'I'
    }

    fn attributes(&self) -> u8 {
        0
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

// Implement NumericOps for TypeInts (element-wise operations)
impl NumericOps for TypeInts {
    #[inline]
    fn add(self, rhs: Self) -> Self {
        TypeInts(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        TypeInts(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        TypeInts(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a * b)
                .collect(),
        )
    }

    #[inline]
    fn div(self, rhs: Self) -> Self {
        TypeInts(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a / b)
                .collect(),
        )
    }
}

// Std ops for convenience
impl Add for TypeInts {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        NumericOps::add(self, rhs)
    }
}

impl Sub for TypeInts {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        NumericOps::sub(self, rhs)
    }
}

impl Mul for TypeInts {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        NumericOps::mul(self, rhs)
    }
}

impl Div for TypeInts {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        NumericOps::div(self, rhs)
    }
}

// Efficient broadcasting: vector operates with float scalar (promotes ints to floats)
impl crate::type_system::BroadcastOps<TypeFloat> for TypeInts {
    type Output = TypeFloats;

    #[inline]
    fn broadcast_add(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x as f32 + scalar.0).collect())
    }

    #[inline]
    fn broadcast_sub(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x as f32 - scalar.0).collect())
    }

    #[inline]
    fn broadcast_mul(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x as f32 * scalar.0).collect())
    }

    #[inline]
    fn broadcast_div(self, scalar: TypeFloat) -> Self::Output {
        TypeFloats(self.0.into_iter().map(|x| x as f32 / scalar.0).collect())
    }
}

impl Promote<TypeFloats> for TypeInts {
    type Output = TypeFloats;

    #[inline]
    fn promote_pair(self, rhs: TypeFloats) -> (TypeFloats, TypeFloats) {
        let promoted_lhs: Vec<f32> = self.0.iter().map(|&x| x as f32).collect();
        (TypeFloats(promoted_lhs), rhs)
    }
}

// Efficient broadcasting: vector operates with scalar (no allocation of broadcast vector)
impl crate::type_system::BroadcastOps<TypeInt> for TypeInts {
    type Output = TypeInts;

    #[inline]
    fn broadcast_add(self, scalar: TypeInt) -> Self::Output {
        TypeInts(self.0.into_iter().map(|x| x + scalar.0).collect())
    }

    #[inline]
    fn broadcast_sub(self, scalar: TypeInt) -> Self::Output {
        TypeInts(self.0.into_iter().map(|x| x - scalar.0).collect())
    }

    #[inline]
    fn broadcast_mul(self, scalar: TypeInt) -> Self::Output {
        TypeInts(self.0.into_iter().map(|x| x * scalar.0).collect())
    }

    #[inline]
    fn broadcast_div(self, scalar: TypeInt) -> Self::Output {
        TypeInts(self.0.into_iter().map(|x| x / scalar.0).collect())
    }
}

// Reverse broadcasting: scalar operates with vector (scalar op vector)
impl crate::type_system::ReverseBroadcastOps<TypeInts> for TypeInt {
    type Output = TypeInts;

    #[inline]
    fn rbroadcast_add(self, vector: TypeInts) -> Self::Output {
        TypeInts(vector.0.into_iter().map(|x| self.0 + x).collect())
    }

    #[inline]
    fn rbroadcast_sub(self, vector: TypeInts) -> Self::Output {
        TypeInts(vector.0.into_iter().map(|x| self.0 - x).collect())
    }

    #[inline]
    fn rbroadcast_mul(self, vector: TypeInts) -> Self::Output {
        TypeInts(vector.0.into_iter().map(|x| self.0 * x).collect())
    }

    #[inline]
    fn rbroadcast_div(self, vector: TypeInts) -> Self::Output {
        TypeInts(vector.0.into_iter().map(|x| self.0 / x).collect())
    }
}

// No promotion needed when both are TypeInts
impl Promote<TypeInts> for TypeInts {
    type Output = TypeInts;

    #[inline]
    fn promote_pair(self, rhs: TypeInts) -> (TypeInts, TypeInts) {
        (self, rhs)
    }
}
