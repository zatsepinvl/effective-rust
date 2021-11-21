use std::fmt::Display;

pub trait CorePrinter {
    fn print(obj: &dyn Display);
}