use app_core::Log;
use app_core::Runtime;

mod my;

pub trait Config: app_core::Config {}

pub struct Plugin<T>(T);

fn private_fn() {
    println!("print from private fn");
}

impl<T: Config> Plugin<T> {
    pub fn run() {
        let context = T::Runtime::context();
        T::Log::info(&context.version);
        private_fn();
        my::public::my_public_fn();
    }
}