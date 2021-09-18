use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display, Formatter};

fn main() {
    from_and_into();
    try_from_and_try_into();
    to_and_from_strings();
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


    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    trait MyTryInto<T>: Sized {
        type Error;

        fn my_try_into(self) -> Result<T, Self::Error>;
    }

    impl<F, T: TryFrom<F>> MyTryInto<T> for F {
        type Error = T::Error;

        fn my_try_into(self) -> Result<T, T::Error> {
            T::try_from(self)
        }
    }

    assert_eq!(EvenNumber::try_from(2), Ok(EvenNumber(2)));
    assert_eq!(EvenNumber::try_from(3), Err(()));

    let event_number_1: EvenNumber = 2i32.try_into().unwrap();
    let event_number_2: EvenNumber = 2i32.my_try_into().unwrap();
    assert_eq!(event_number_1, event_number_2);
}


fn to_and_from_strings() {
    println!("\n# to_and_from_strings");

    struct Circle {
        radius: i32,
    }
    impl Display for Circle {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    println!("{}", Circle { radius: 1 }.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
