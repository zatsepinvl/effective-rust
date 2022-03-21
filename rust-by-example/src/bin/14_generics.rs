use std::fmt::Debug;


fn main() {
    params_and_struct();
    traits();
    newtype_idiom();
    default_type_param();
    associated_types();
}

fn params_and_struct() {
    #[derive(Debug)]
    struct A;

    #[derive(Debug)]
    struct GenericA<T: Debug>(T);

    impl<T: Debug> GenericA<T> {
        fn log(&self) {
            println!("{:?}", self);
        }
    }

    impl GenericA<i32> {}

    fn foo<T>(arg: T)
        where T: Debug
    {
        println!("{:?}", arg);
    }

    foo(A {});
    foo(GenericA(A {}));

    GenericA(A {}).log();

    let a = &1;
    let &b = a;
}

fn traits() {
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {
            println!("Values dropped");
        }
    }

    //let str = "str";
    let string = String::from("string");
    //str.drop();
    let another = 1;
    string.double_drop(another);
}

// https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
fn newtype_idiom() {
    #[derive(Debug)]
    struct Years(i32);

    #[derive(Debug)]
    struct Days(i32);

    impl From<Days> for Years {
        fn from(days: Days) -> Self {
            Years(days.0 / 365)
        }
    }
    impl From<Years> for Days {
        fn from(years: Years) -> Self {
            Days(years.0 * 365)
        }
    }

    let days = Days(366);
    let years: Years = days.into();

    println!("{:?}", years);
}

fn default_type_param() {
    #[derive(Debug)]
    struct Bar {}

    trait Foo<T = Bar>
        where T: Debug {
        fn log(arg: &T);
    }

    impl Foo for Bar {
        fn log(arg: &Bar) {
            println!("{:?}", arg);
        }
    }

    let bar = Bar {};
    Bar::log(&bar);
}

fn associated_types() {
    mod module {
        #[derive(Debug)]
        pub struct Foo(i32);

        pub struct Bar {
            count: i32,
            current: i32,
        }

        impl Bar {
            pub fn new(count: i32) -> Bar {
                Bar { count, current: 0 }
            }
        }

        impl Iterator for Bar {
            type Item = Foo;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current < self.count {
                    let temp = self.current;
                    self.current += 1;
                    Some(Foo(temp))
                } else {
                    None
                }
            }
        }
    }

    let bar = module::Bar::new(3);

    for i in bar {
        println!("{:?}", i);
    }
}



