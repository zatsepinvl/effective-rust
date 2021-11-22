use std::fmt::{Debug};

pub struct Context {
    pub version: &'static str,
}

pub trait Runtime {
    fn context() -> &'static Context;
}

pub trait Log {
    fn info(obj: &dyn Debug);
}

pub trait Config {
    type Runtime: Runtime;
    type Log: Log;
}