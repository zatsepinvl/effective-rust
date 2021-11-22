use std::fmt::{Debug};
use app_core::{Config, Context};

const CONTEXT: Context = Context { version: "123" };

struct App;

impl app_core::Runtime for App {
    fn context() -> &'static Context {
        return &CONTEXT;
    }
}

impl app_core::Log for App {
    fn info(obj: &dyn Debug) {
        println!("[INFO] {:?}", obj);
    }
}

impl Config for App {
    type Runtime = Self;
    type Log = Self;
}

impl plugin::Config for App {}

fn main() {
    plugin::Plugin::<App>::run();
}