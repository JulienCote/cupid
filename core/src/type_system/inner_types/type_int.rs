use crate::type_system::{NumericOps, Promote, Type, inner_types::TypeFloat};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub struct TypeInt(pub i32);

impl From<i32> for TypeInt {
    fn from(value: i32) -> Self {
        TypeInt(value)
    }
}

impl Type for TypeInt {
    fn name(&self) -> &str {
        "Int"
    }

    fn type_id(&self) -> char {
        'i'
    }

    fn attributes(&self) -> u8 {
        0
    }

    fn size(&self) -> usize {
        1
    }
}

// Implement NumericOps for TypeInt (opt-in to numeric operations)
impl NumericOps for TypeInt {
    #[inline]
    fn add(self, rhs: Self) -> Self {
        TypeInt(self.0 + rhs.0)
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        TypeInt(self.0 - rhs.0)
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        TypeInt(self.0 * rhs.0)
    }

    #[inline]
    fn div(self, rhs: Self) -> Self {
        TypeInt(self.0 / rhs.0)
    }
}

// Std ops for convenience
impl Add for TypeInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        NumericOps::add(self, rhs)
    }
}

impl Sub for TypeInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        NumericOps::sub(self, rhs)
    }
}

impl Mul for TypeInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        NumericOps::mul(self, rhs)
    }
}

impl Div for TypeInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        NumericOps::div(self, rhs)
    }
}

// Promote TypeInt to TypeFloat when operating with floats
impl Promote<TypeFloat> for TypeInt {
    type Output = TypeFloat;

    #[inline]
    fn promote_pair(self, rhs: TypeFloat) -> (TypeFloat, TypeFloat) {
        (TypeFloat(self.0 as f32), rhs)
    }
}

// No promotion needed when both are TypeInt
impl Promote<TypeInt> for TypeInt {
    type Output = TypeInt;

    #[inline]
    fn promote_pair(self, rhs: TypeInt) -> (TypeInt, TypeInt) {
        (self, rhs)
    }
}

// Reverse broadcasting: int scalar operates with float vector (promotes int to float)
impl crate::type_system::ReverseBroadcastOps<crate::type_system::inner_types::TypeFloats>
    for TypeInt
{
    type Output = crate::type_system::inner_types::TypeFloats;

    #[inline]
    fn rbroadcast_add(self, vector: crate::type_system::inner_types::TypeFloats) -> Self::Output {
        let s = self.0 as f32;
        crate::type_system::inner_types::TypeFloats(vector.0.into_iter().map(|x| s + x).collect())
    }

    #[inline]
    fn rbroadcast_sub(self, vector: crate::type_system::inner_types::TypeFloats) -> Self::Output {
        let s = self.0 as f32;
        crate::type_system::inner_types::TypeFloats(vector.0.into_iter().map(|x| s - x).collect())
    }

    #[inline]
    fn rbroadcast_mul(self, vector: crate::type_system::inner_types::TypeFloats) -> Self::Output {
        let s = self.0 as f32;
        crate::type_system::inner_types::TypeFloats(vector.0.into_iter().map(|x| s * x).collect())
    }

    #[inline]
    fn rbroadcast_div(self, vector: crate::type_system::inner_types::TypeFloats) -> Self::Output {
        let s = self.0 as f32;
        crate::type_system::inner_types::TypeFloats(vector.0.into_iter().map(|x| s / x).collect())
    }
}
