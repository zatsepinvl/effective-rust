use std::fmt::Display;

use appcore;
use appcore::CorePrinter;

struct Runtime;

impl CorePrinter for Runtime {
    fn print(obj: &dyn Display) {
        println!("CUSTOM PRINT {}", obj);
    }
}

impl plugin::Config for Runtime {
    type Printer = Self;
}

fn main() {
    plugin::Plugin::<Runtime>::print(1);
}