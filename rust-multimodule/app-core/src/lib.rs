use std::fmt::{Debug};

pub struct Context {
    pub version: &'static str,
}

pub trait ContextProvider {
    fn context() -> &'static Context;
}

pub trait Log {
    fn info(obj: &dyn Debug);
}

pub trait Config {
    type ContextProvider: ContextProvider;
    type Log: Log;
}
