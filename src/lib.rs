pub mod prelude {
    pub use async_std::prelude::*;
    pub use htmx_comp_macro::{global,htmx_comp};
    pub use lazy_static::lazy_static;
    pub use std::sync::{ Arc, Mutex };
    pub use maud::html;
    pub use crate::{server, lock_globals};
    pub struct Global<T> {
        inner: Arc<Mutex<T>>,
    }

    impl<T> Global<T> {
        pub fn new(initial_value: T) -> Self {
            Global {
                inner: Arc::new(Mutex::new(initial_value)),
            }
        }

        pub fn clone(&self) -> Self {
            Global {
                inner: self.inner.clone(),
            }
        }

        pub fn lock(&self) -> Result<std::sync::MutexGuard<'_, T>, std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> {
            self.inner.lock()
        }

        pub fn try_lock(&self) -> Result<std::sync::MutexGuard<'_, T>, std::sync::TryLockError<std::sync::MutexGuard<'_, T>>>{
            self.inner.try_lock()
        }
    }
}

pub mod macros;
