pub mod prelude {
    pub use async_std::prelude::*;
    pub use htmx_comp_macro::{global,htmx_comp};
    //pub use server, lock_globals
    pub use lazy_static::lazy_static;
    pub use std::sync::{ Arc, Mutex };
    pub use maud::html;
    pub use crate::{server, lock_globals};
}

pub mod macros;
