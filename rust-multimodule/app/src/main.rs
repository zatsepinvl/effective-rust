use std::fmt::{Debug};
use app_core::{Config, Context};

const CONTEXT: Context = Context { version: "123" };

struct Runtime {}

impl app_core::ContextProvider for Runtime {
    fn context() -> &'static Context {
        return &CONTEXT;
    }
}

impl app_core::Log for Runtime {
    fn info(obj: &dyn Debug) {
        println!("[INFO] {:?}", obj);
    }
}

impl Config for Runtime {
    type ContextProvider = Self;
    type Log = Self;
}

impl plugin::Config for Runtime {}

fn main() {
    plugin::Plugin::<Runtime>::run();
}
