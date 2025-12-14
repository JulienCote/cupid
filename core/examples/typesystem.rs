use cupid_core::lang::{
    invokable::operator::{OperatorAdd, OperatorDivide, OperatorSubtract},
    type_system::{TypeBool, TypeByte, TypeFloat, TypeInt},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Cupid Type System Example ---");

    // 1. Integer Arithmetic
    let int1 = TypeInt(10);
    let int2 = TypeInt(20);
    let int_sum = TypeInt::add(&int1, &int2)?;
    println!("Int: {:?} + {:?} = {:?}", int1, int2, int_sum);

    let int_sub = TypeInt::sub(&int2, &int1)?;
    println!("Int: {:?} - {:?} = {:?}", int2, int1, int_sub);

    // 2. Float Arithmetic
    let float1 = TypeFloat(10.5);
    let float2 = TypeFloat(2.5);
    let float_div = TypeFloat::div(&float1, &float2)?;
    println!("Float: {:?} / {:?} = {:?}", float1, float2, float_div);

    // 3. Bool Arithmetic (Promotion to Int)
    let bool1 = TypeBool(true); // 1
    let bool2 = TypeBool(true); // 1
    let bool_sum: TypeInt = TypeBool::add(&bool1, &bool2)?;
    println!(
        "Bool: {:?} + {:?} = {:?} (Promoted to Int)",
        bool1, bool2, bool_sum
    );
    assert_eq!(bool_sum, TypeInt(2));

    let bool3 = TypeBool(false); // 0
    let bool_sum2: TypeInt = TypeBool::add(&bool1, &bool3)?;
    println!(
        "Bool: {:?} + {:?} = {:?} (Promoted to Int)",
        bool1, bool3, bool_sum2
    );
    assert_eq!(bool_sum2, TypeInt(1));

    // 4. Byte Arithmetic (Promotion to Int)
    let byte1 = TypeByte(100);
    let byte2 = TypeByte(50);
    let byte_sum: TypeInt = TypeByte::add(&byte1, &byte2)?;
    println!(
        "Byte: {:?} + {:?} = {:?} (Promoted to Int)",
        byte1, byte2, byte_sum
    );
    assert_eq!(byte_sum, TypeInt(150));

    // 5. Casting
    let int_val = TypeInt(42);
    let float_val: TypeFloat = int_val.into();
    println!("Cast Int {:?} to Float {:?}", int_val, float_val);
    assert_eq!(float_val, TypeFloat(42.0));

    let float_val2 = TypeFloat(123.456);
    let int_val2: TypeInt = float_val2.into();
    println!("Cast Float {:?} to Int {:?}", float_val2, int_val2);
    assert_eq!(int_val2, TypeInt(123)); // 123.456 rounds to 123

    let float_val3 = TypeFloat(123.6);
    let int_val3: TypeInt = float_val3.into();
    println!("Cast Float {:?} to Int {:?}", float_val3, int_val3);
    assert_eq!(int_val3, TypeInt(124)); // 123.6 rounds to 124

    println!("\nType System is working as expected!");

    Ok(())
}
