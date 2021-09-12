fn main() {
    from_and_into();
    try_from_and_try_into();
}

fn from_and_into() {
    println!("\n# from_and_into");
    //str -> String
    let str = "str";
    let string: String = String::from(str);
    println!("{} = {}", str, string);

    #[derive(Debug)]
    struct Number(u32);

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            return Number(value as u32);
        }
    }

    impl Into<i32> for Number {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }

    let number_from_i32: Number = Number::from(1i32);
    // For free - From implies Into
    let i32_into_number: Number = 1i32.into();
    let number_into_i32: i32 = Number(1).into();

    println!("To {:?} from {:?} and into {:?}", number_from_i32, number_into_i32, i32_into_number);
}

fn try_from_and_try_into() {
    println!("\n# try_from_and_try_into");
}

