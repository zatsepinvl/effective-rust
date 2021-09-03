fn main() {
    println!("PRIMITIVES");

    scalar_types();
}

fn scalar_types() {
    println!("\n# Integers");
    let a_i8: i8 = 127; // -128..127
    let a_i16: i16 = -32768; // -32768..32767
    let a_i32: i32 = i32::MIN;
    let a_i64: i64 = i64::MAX;
    let a_i128: i128 = i128::MAX;
    let a_isize: isize = isize::MAX;
    // the same for u8, u16, u32, u64, u128, usize

    println!("i8    : {}", a_i8);
    println!("i16   : {}", a_i16);
    println!("i32   : {}", a_i32);
    println!("i64   : {}", a_i64);
    println!("i128  : {}", a_i128);
    println!("isize : {}", a_isize);

    println!("\n# Float point");
    let a_f32: f32 = f32::MAX;
    let a_f64: f64 = 1.43123123;
    println!("f32   : {}", a_f32);
    println!("f64   : {}", a_f64);

    println!("\n# Char");
    let a_char: char = '∞';
    println!("char  : {}", a_char);

    println!("\n# Bool");
    let a_bool: bool = true;
    println!("char  : {}", a_bool);

    println!("\n# Unit type");
    let a_unit: () = ();
    println!("unit  : {:?}", a_unit);
}