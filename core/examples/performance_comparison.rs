use cupid_core::type_system::{SuperType, TypeFloat, TypeInt, TypeInts, operators::Operator};
use std::time::{Duration, Instant};

const SCALAR_ITERATIONS: usize = 10_000_000;
const VECTOR_ITERATIONS: usize = 5_000;
const VECTOR_SIZE: usize = 10_000;

fn main() {
    println!("=== Performance Comparison: Native Rust vs Cupid Type System ===\n");
    println!("Scalar Iterations: {}", SCALAR_ITERATIONS);
    println!("Vector Iterations: {}", VECTOR_ITERATIONS);
    println!("Vector Size:       {}\n", VECTOR_SIZE);

    benchmark_scalar_add();
    benchmark_vector_add();
    benchmark_broadcast_add();
    benchmark_broadcast_promotion();
    benchmark_scalar_accumulation();
}

fn print_comparison(native: Duration, cupid: Duration) {
    let ratio = cupid.as_secs_f64() / native.as_secs_f64();
    println!("Ratio (Cupid / Native): {:.2}x", ratio);
    if ratio < 1.0 {
        println!("(Cupid is faster!)");
    } else {
        println!("(Native is faster)");
    }
}

fn benchmark_scalar_add() {
    println!("--- 1. Scalar Addition (Int + Int) ---");

    // Native
    let a = 10;
    let b = 20;
    let mut sum = 0;
    let start = Instant::now();
    for _ in 0..SCALAR_ITERATIONS {
        // Prevent optimization
        sum += std::hint::black_box(a) + std::hint::black_box(b);
    }
    let native_duration = start.elapsed();
    println!("Native Rust (i32): {:?}", native_duration);
    if sum == 0 {
        println!("")
    };

    // Cupid
    let a = SuperType(TypeInt(10));
    let b = SuperType(TypeInt(20));
    let mut last_result = SuperType(TypeInt(0));
    let start = Instant::now();
    for _ in 0..SCALAR_ITERATIONS {
        // Note: TypeInt is Copy/Clone cheap, so cloning is fine
        last_result = Operator::Add.apply(
            std::hint::black_box(a.clone()),
            std::hint::black_box(b.clone()),
        );
    }
    let cupid_duration = start.elapsed();
    println!("Cupid Type System: {:?}", cupid_duration);

    // Use result to prevent optimization
    if last_result.0.0 == 0 {
        println!("")
    };

    print_comparison(native_duration, cupid_duration);
}

fn benchmark_vector_add() {
    println!("\n--- 2. Vector Addition (Ints + Ints) ---");

    let vec_a: Vec<i32> = (0..VECTOR_SIZE as i32).collect();
    let vec_b: Vec<i32> = (0..VECTOR_SIZE as i32).map(|x| x * 2).collect();

    // Native
    // We clone inputs to match Cupid's value semantics (consuming inputs)
    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        let _res: Vec<i32> = std::hint::black_box(vec_a.clone())
            .into_iter()
            .zip(std::hint::black_box(vec_b.clone()).into_iter())
            .map(|(a, b)| a + b)
            .collect();
        std::hint::black_box(_res);
    }
    let native_duration = start.elapsed();
    println!("Native Rust (Vec<i32>): {:?}", native_duration);

    // Cupid
    let a = SuperType(TypeInts(vec_a.clone()));
    let b = SuperType(TypeInts(vec_b.clone()));

    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        let _res = Operator::Add.apply(
            std::hint::black_box(a.clone()),
            std::hint::black_box(b.clone()),
        );
        std::hint::black_box(_res);
    }
    let cupid_duration = start.elapsed();
    println!("Cupid Type System: {:?}", cupid_duration);

    print_comparison(native_duration, cupid_duration);
}

fn benchmark_broadcast_add() {
    println!("\n--- 3. Broadcast Addition (Int + Ints) ---");

    let scalar = 10;
    let vec_a: Vec<i32> = (0..VECTOR_SIZE as i32).collect();

    // Native
    // We clone vector to match Cupid's value semantics
    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        let _res: Vec<i32> = std::hint::black_box(vec_a.clone())
            .into_iter()
            .map(|x| x + std::hint::black_box(scalar))
            .collect();
        std::hint::black_box(_res);
    }
    let native_duration = start.elapsed();
    println!("Native Rust (i32 + Vec<i32>): {:?}", native_duration);

    // Cupid
    let s = SuperType(TypeInt(scalar));
    let v = SuperType(TypeInts(vec_a.clone()));

    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        // Reverse broadcast: Scalar + Vector
        let _res = Operator::Add.apply(
            std::hint::black_box(s.clone()),
            std::hint::black_box(v.clone()),
        );
        std::hint::black_box(_res);
    }
    let cupid_duration = start.elapsed();
    println!("Cupid Type System: {:?}", cupid_duration);

    print_comparison(native_duration, cupid_duration);
}

fn benchmark_broadcast_promotion() {
    println!("\n--- 4. Broadcast Promotion (Float + Ints) ---");

    let scalar = 2.5f32;
    let vec_a: Vec<i32> = (0..VECTOR_SIZE as i32).collect();

    // Native
    // We clone vector to match Cupid's value semantics
    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        let _res: Vec<f32> = std::hint::black_box(vec_a.clone())
            .into_iter()
            .map(|x| std::hint::black_box(scalar) + x as f32)
            .collect();
        std::hint::black_box(_res);
    }
    let native_duration = start.elapsed();
    println!("Native Rust (f32 + Vec<i32>): {:?}", native_duration);

    // Cupid
    let s = SuperType(TypeFloat(scalar));
    let v = SuperType(TypeInts(vec_a.clone()));

    let start = Instant::now();
    for _ in 0..VECTOR_ITERATIONS {
        // Reverse broadcast with promotion: Float + Ints
        let _res = Operator::Add.apply(
            std::hint::black_box(s.clone()),
            std::hint::black_box(v.clone()),
        );
        std::hint::black_box(_res);
    }
    let cupid_duration = start.elapsed();
    println!("Cupid Type System: {:?}", cupid_duration);

    print_comparison(native_duration, cupid_duration);
}

fn benchmark_scalar_accumulation() {
    println!("\n--- 5. Scalar Accumulation (Int += Int) ---");
    let iterations = 100_000_000;

    // Native
    let mut sum = 0;
    let val = 1;
    let start = Instant::now();
    for _ in 0..iterations {
        sum += std::hint::black_box(val);
    }
    let native_duration = start.elapsed();
    println!("Sum: {}", sum);
    println!("Native Rust (i32 += i32): {:?}", native_duration);
    std::hint::black_box(sum);

    // Cupid
    let mut sum = SuperType(TypeInt(0));
    let val = SuperType(TypeInt(1));

    let start = Instant::now();
    for _ in 0..iterations {
        sum = Operator::Add.apply(sum, std::hint::black_box(val.clone()));
    }
    let cupid_duration = start.elapsed();
    println!("Sum: {}", sum.0.0);
    println!("Cupid Type System: {:?}", cupid_duration);

    print_comparison(native_duration, cupid_duration);
}
