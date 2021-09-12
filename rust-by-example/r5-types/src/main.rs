// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

/**
 * Rust provides no implicit type conversion (coercion) between primitive types.
 * But, explicit type conversion (casting) can be performed using the **as** keyword.
 * Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior.
 * The behavior of all casts between integral types is well defined in Rust.
 **/
fn casting() {
    println!("\n# casting");

    let float = 65.4321_f32;

    // Error! No implicit conversion
    //let integer: u8 = decimal;

    // Explicit conversion
    let integer = float as u8;
    let character = integer as char;

    // A float cannot be directly converted to a char.
    //let character = float as char;
    let character = float as u8 as char;

    println!("Casting: {} -> {} -> {}", float, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);
    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!(" 129 as a i8 is : {}", 128 as i8);
    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn literals() {
    println!("\n# literals");
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    println!("\n# inference");
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}

fn aliasing() {
    println!("\n# aliasing");

    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning.
    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanoseconds: NanoSecond = 100 as u64_t;
    let inches: Inch = 2 as u64_t;
    let sum = nanoseconds + inches;

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    println!("{} nanoseconds + {} inches = {} {}", nanoseconds, inches, sum, type_of(&sum));
}
