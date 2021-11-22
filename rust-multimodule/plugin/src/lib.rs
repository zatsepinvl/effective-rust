use app_core::Log;
use app_core::Runtime;

pub trait Config: app_core::Config {}

pub struct Plugin<T>(T);

impl<T: Config> Plugin<T> {
    pub fn run() {
        let context = T::Runtime::context();
        T::Log::info(&context.version);
    }
}