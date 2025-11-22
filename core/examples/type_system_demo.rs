use cupid_core::type_system::{SuperType, TypeFloat, TypeInt, TypeInts, operators::Operator};

fn main() {
    println!("=== Flexible Type System Demo ===\n");

    // Same type operations
    println!("1. Same type operations:");
    let a = SuperType(TypeInt(10));
    let b = SuperType(TypeInt(32));
    let result = Operator::Add.apply(a, b);
    println!("   10 + 32 = {}", result.0.0);

    // Type promotion: Int + Float -> Float
    println!("\n2. Automatic type promotion (Int + Float):");
    let int_val = SuperType(TypeInt(10));
    let float_val = SuperType(TypeFloat(32.5));
    let result = Operator::Add.apply(int_val, float_val);
    println!("   10 + 32.5 = {}", result.0.0);

    // Reverse: Float + Int -> Float
    println!("\n3. Commutative operations (Float + Int):");
    let float_val = SuperType(TypeFloat(10.5));
    let int_val = SuperType(TypeInt(32));
    let result = Operator::Add.apply(float_val, int_val);
    println!("   10.5 + 32 = {}", result.0.0);

    // Scalar broadcasting: Int + Ints -> Ints
    println!("\n4. Scalar broadcast to vector (Int + Ints):");
    let scalar = SuperType(TypeInt(10));
    let vector = SuperType(TypeInts(vec![1, 2, 3, 4, 5]));
    let result = Operator::Add.apply(scalar, vector);
    println!("   10 + [1,2,3,4,5] = {:?}", result.0.0);

    // Reverse: Ints + Int -> Ints
    println!("\n5. Vector with scalar (Ints * Int):");
    let vector = SuperType(TypeInts(vec![2, 4, 6, 8]));
    let scalar = SuperType(TypeInt(3));
    let result = Operator::Multiply.apply(vector, scalar);
    println!("   [2,4,6,8] * 3 = {:?}", result.0.0);

    // Float + Ints -> TypeFloats (with promotion and broadcast)
    println!("\n6. Float scalar broadcast to int vector:");
    let scalar = SuperType(TypeFloat(2.5));
    let vector = SuperType(TypeInts(vec![10, 20, 30]));
    let result = Operator::Multiply.apply(scalar, vector);
    println!("   2.5 * [10,20,30] = {:?}", result.0.0);

    // Vector operations
    println!("\n7. Element-wise vector operations:");
    let vec_a = SuperType(TypeInts(vec![1, 2, 3]));
    let vec_b = SuperType(TypeInts(vec![10, 20, 30]));
    let result = Operator::Add.apply(vec_a, vec_b);
    println!("   [1,2,3] + [10,20,30] = {:?}", result.0.0);

    println!("\n=== Key Features ===");
    println!("✓ Only implement operations once (not Int+Float AND Float+Int)");
    println!("✓ Automatic type promotion (Int -> Float when needed)");
    println!("✓ Scalar broadcasting to vectors (Atom + List)");
    println!("✓ Opt-in operators via NumericOps trait");
    println!("✓ Types without NumericOps (like Lambda) can skip operators");
}
