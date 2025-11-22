# Type System Design

This is a flexible type system implementation for a Q-like array language, using the **state type pattern** in Rust.

## Key Design Goals

1. **Only implement operations once** - No need to write both `Int + Float` and `Float + Int`
2. **Opt-in operators** - Types like `Lambda` don't need to implement numeric operations
3. **Automatic type promotion** - `Int` automatically promotes to `Float` when needed
4. **Scalar broadcasting** - Adding an atom to a list broadcasts the atom to each element

## Architecture

### Core Traits

- **`Type`** - Base trait for all types with metadata (name, id, attributes, size)
- **`NumericOps`** - Opt-in trait for types supporting numeric operations
- **`Promote<Rhs>`** - Enables automatic type promotion and broadcasting

### Type Categories

- **`Atom`** - Scalar values (Int, Float, etc.)
- **`List`** - Vector values (Ints, Floats, etc.)

### How It Works

#### 1. Opt-in Operators via NumericOps

Types implement `NumericOps` to support arithmetic:

```rust
impl NumericOps for TypeInt {
    fn add(self, rhs: Self) -> Self { TypeInt(self.0 + rhs.0) }
    fn sub(self, rhs: Self) -> Self { TypeInt(self.0 - rhs.0) }
    fn mul(self, rhs: Self) -> Self { TypeInt(self.0 * rhs.0) }
    fn div(self, rhs: Self) -> Self { TypeInt(self.0 / rhs.0) }
}
```

Types that don't need operators (like `Lambda`) simply don't implement this trait.

#### 2. Automatic Type Promotion

The `Promote` trait handles type conversions:

```rust
// Int + Float -> Float (Int promotes to Float)
impl Promote<TypeFloat> for TypeInt {
    type Output = TypeFloat;
    fn promote_pair(self, rhs: TypeFloat) -> (TypeFloat, TypeFloat) {
        (TypeFloat(self.0 as f32), rhs)
    }
}

// Float + Int -> Float (Int promotes to Float)  
impl Promote<TypeInt> for TypeFloat {
    type Output = TypeFloat;
    fn promote_pair(self, rhs: TypeInt) -> (TypeFloat, TypeFloat) {
        (self, TypeFloat(rhs.0 as f32))
    }
}
```

Now you only need to implement the promotion in both directions, and operators work commutatively!

#### 3. Scalar Broadcasting

When an atom operates with a list, the atom broadcasts to each element:

```rust
// Int + Ints -> Ints (broadcast Int to each element)
impl Promote<TypeInts> for TypeInt {
    type Output = TypeInts;
    fn promote_pair(self, rhs: TypeInts) -> (TypeInts, TypeInts) {
        let broadcast = vec![self.0; rhs.0.len()];
        (TypeInts(broadcast), rhs)
    }
}
```

Examples:
- `10 + [1, 2, 3]` → `[11, 12, 13]`
- `2.5 * [10, 20, 30]` → `[25.0, 50.0, 75.0]`

## Usage

### Same Type Operations

```rust
let a = SuperType(TypeInt(10));
let b = SuperType(TypeInt(32));
let result = Operator::Add.apply(a, b);
// result = 42
```

### Operations with Type Promotion

```rust
let int_val = SuperType(TypeInt(10));
let float_val = SuperType(TypeFloat(32.5));
let result = Operator::Add.apply_with_promotion(int_val, float_val);
// result = 42.5 (TypeFloat)
```

### Scalar Broadcasting

```rust
let scalar = SuperType(TypeInt(10));
let vector = SuperType(TypeInts(vec![1, 2, 3]));
let result = Operator::Add.apply_with_promotion(scalar, vector);
// result = [11, 12, 13] (TypeInts)
```

## Adding New Types

To add a new type that supports operators:

1. Define your type struct
2. Implement `Type` trait
3. Implement `NumericOps` trait (if you want operators)
4. Implement `Promote<T>` for each type you want to interact with

To add a type that doesn't support operators (like Lambda):

1. Define your type struct
2. Implement `Type` trait
3. Done! No need to implement `NumericOps`

## Type Promotion Rules

Current promotion hierarchy:
- `Int` → `Float` (when operating with Float)
- `Ints` → `Floats` (when operating with Float types)
- `Atom` → `List` (broadcast atom to list length)

## Running Tests

```bash
cargo test --lib type_system
```

## Running the Demo

```bash
cargo run --example type_system_demo
```
