use std::fmt::{Display, Formatter};
use std::mem;

fn main() {
    println!("PRIMITIVES");

    scalar_types();
    literals_and_operators();
    tuples();
    arrays_and_slices();
}

#[allow(dead_code)]
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

    println!("\n# Array");
    let a_array = [1, 2, 3];
    println!("array  : {:?}", a_array);

    println!("\n# Tuple");
    let a_tuple = (1, 2, "n");
    println!("tuple {:?}", a_tuple);

    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}

fn literals_and_operators() {
    // 0x - hexadecimal
    // 0o - octal
    // 0b - binary

    println!("\n # Literals and operators");
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // causes overflow
    //println!("1 - 2 = {}", 1u32 - 2);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("1 >> 5 is {}", 1u32 >> 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

fn tuples() {
    println!("\n# Tuples");

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        (pair.1, pair.0)
    }
    println!("reverse {:?}", reverse((1, true)));

    let tuple_of_tuples = ((0, 1), (true, false), ("yes", "no"));
    println!("tuple-of-tuples {:?}", tuple_of_tuples);

    // destruction
    let (a, b, c) = tuple_of_tuples;
    println!("destructed {:?}", a);

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f64);
    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "( {} {} )", self.0, self.1);
            write!(f, "( {} {} )", self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    fn transpose(matrix: &Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(&matrix));
    // Will be 24 bytes, not (4+4+4+8=)20 bytes because of padding on 64-bit machine:
    // https://stackoverflow.com/questions/65707981/why-is-the-size-of-a-tuple-or-struct-not-the-sum-of-the-members
    println!("Matrix occupies {} bytes", mem::size_of_val(&matrix));
}

fn arrays_and_slices() {
    println!("\n# Arrays and slices");

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    fn first_word_wrong(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    fn first_word_right(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let mut str = String::from("fff first word");
    let first_word_index = first_word_wrong(&str);
    let first_word = first_word_right(&str);
    println!("First word: {}", first_word);
    // Will cause "byte index 3 is out of bounds of ``" error
    // str.clear();
    // println!("First word: {}", &str[..first_word_index]);
}