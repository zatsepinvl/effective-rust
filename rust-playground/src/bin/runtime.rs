use std::marker::PhantomData;
use crate::module_a::Pallet;

fn main() {
    struct Runtime;
    struct ConsoleLogger;

    impl Logger for ConsoleLogger {
        fn log() {
            println!("Log1")
        }
    }

    impl module_a::Config for Runtime {
        type MyLogger = ConsoleLogger;
    }

    let pallet: Pallet<Runtime> = Pallet(PhantomData);
    pallet.do_something();
}


pub trait Logger {
    fn log();
}

mod module_a {
    use std::marker::PhantomData;
    use crate::Logger;

    pub trait Config {
        type MyLogger: Logger;
    }

    pub struct Pallet<T>(pub PhantomData<T>);

    impl<T: Config> Pallet<T> {
        pub fn do_something(&self) {
            T::MyLogger::log();
        }
    }
}
