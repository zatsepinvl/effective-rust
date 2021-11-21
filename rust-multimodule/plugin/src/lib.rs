use appcore::CorePrinter;

pub trait Config {
    type Printer: CorePrinter;
}

pub struct Plugin<T>(T);

impl<T: Config> Plugin<T> {
    pub fn print(i: u32) {
        T::Printer::print(&i);
    }
}