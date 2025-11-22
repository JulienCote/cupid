use cupid_core::type_system::{
    SuperType, operators::Operator,
    TypeInt, TypeInts, TypeFloat,
};

fn main() {
    println!("=== Efficient Broadcasting Demo ===\n");
    
    // Example 1: Scalar + Vector (no temporary vector allocation)
    println!("1. Scalar + Vector broadcasting:");
    let scalar = TypeInt(10);
    let vector = TypeInts(vec![1, 2, 3, 4, 5]);
    // Unified apply() handles broadcasting efficiently!
    let result = Operator::Add.apply(SuperType(scalar), SuperType(vector));
    println!("   10 + [1,2,3,4,5] = {:?}", result.0.0);
    println!("   ✓ No temporary broadcast vector allocated!\n");
    
    // Example 2: Vector + Scalar (efficient iteration)
    println!("2. Vector + Scalar broadcasting:");
    let vector = TypeInts(vec![10, 20, 30, 40]);
    let scalar = TypeInt(5);
    let result = Operator::Multiply.apply(SuperType(vector), SuperType(scalar));
    println!("   [10,20,30,40] * 5 = {:?}", result.0.0);
    println!("   ✓ Scalar value used directly in map operation!\n");
    
    // Example 3: Float scalar + Int vector (type promotion + broadcasting)
    println!("3. Float + Int vector (with type promotion):");
    let scalar = TypeFloat(2.5);
    let vector = TypeInts(vec![2, 4, 6, 8]);
    let result = Operator::Multiply.apply(SuperType(scalar), SuperType(vector));
    println!("   2.5 * [2,4,6,8] = {:?}", result.0.0);
    println!("   ✓ Ints promoted to floats during iteration, no temp vector!\n");
    
    // Example 4: Int vector + Float scalar
    println!("4. Int vector + Float scalar:");
    let vector = TypeInts(vec![100, 200, 300]);
    let scalar = TypeFloat(1.5);
    let result = Operator::Multiply.apply(SuperType(vector), SuperType(scalar));
    println!("   [100,200,300] * 1.5 = {:?}", result.0.0);
    println!("   ✓ Elements promoted during iteration!\n");
    
    // Example 5: Vector + Vector (element-wise, no broadcasting)
    println!("5. Vector + Vector (element-wise):");
    let vec1 = TypeInts(vec![1, 2, 3]);
    let vec2 = TypeInts(vec![10, 20, 30]);
    let result = Operator::Add.apply(SuperType(vec1), SuperType(vec2));
    println!("   [1,2,3] + [10,20,30] = {:?}", result.0.0);
    println!("   ✓ Standard element-wise operation\n");
    
    println!("=== Performance Benefits ===");
    println!("✓ No temporary broadcast vectors allocated");
    println!("✓ Scalar values accessed directly (cache-friendly)");
    println!("✓ Type promotion happens during iteration (no intermediate allocations)");
    println!("✓ Same number of final elements allocated as needed");
    println!("✓ Unified API: Operator::apply() handles everything!");
}
