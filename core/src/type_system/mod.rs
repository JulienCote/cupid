mod inner_types;
pub mod operators;
mod super_type;

pub use inner_types::{TypeFloat, TypeFloats, TypeInt, TypeInts};
pub use super_type::SuperType;

/// Base trait for all types in the system
pub trait Type {
    fn name(&self) -> &str;
    fn type_id(&self) -> char;
    fn attributes(&self) -> u8;
    fn size(&self) -> usize;
}

/// Trait for types that can be promoted to a common type
/// This enables automatic type coercion (e.g., Int -> Float)
pub trait Promote<Rhs = Self> {
    type Output: Type;
    fn promote_pair(self, rhs: Rhs) -> (Self::Output, Self::Output);
}

/// Optional trait for types that support numeric operations
/// Types like Lambda can simply not implement this trait
pub trait NumericOps: Type + Sized {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
}

/// Trait for efficient scalar-vector broadcasting operations
/// This allows operations like scalar + vector without allocating a broadcast vector
pub trait BroadcastOps<Scalar>: Type + Sized {
    type Output: Type;
    fn broadcast_add(self, scalar: Scalar) -> Self::Output;
    fn broadcast_sub(self, scalar: Scalar) -> Self::Output;
    fn broadcast_mul(self, scalar: Scalar) -> Self::Output;
    fn broadcast_div(self, scalar: Scalar) -> Self::Output;
}

/// Trait for efficient reverse scalar-vector broadcasting (scalar op vector)
pub trait ReverseBroadcastOps<Vector>: Type + Sized {
    type Output: Type;
    fn rbroadcast_add(self, vector: Vector) -> Self::Output;
    fn rbroadcast_sub(self, vector: Vector) -> Self::Output;
    fn rbroadcast_mul(self, vector: Vector) -> Self::Output;
    fn rbroadcast_div(self, vector: Vector) -> Self::Output;
}
