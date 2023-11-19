# htmx_server

## Overview

`htmx_server` is a Rust crate that simplifies server-side of
[htmx](https://htmx.org/) components, not that it was that hard anyway. This crate is simply meant to 
ship htmx and rust stack apps in the most minimal code possible. This crate is still in very early development.
There are plans to implement a full HTML! macro similar to JSX that is specialized
around HTMX. For now however, this crate it only able to serve raw text without
type verification. Tread carefully!

State management is coming soon

## Getting Started

To use `htmx_server` in your Rust project, add it as a dependency in your
`Cargo.toml` file:

```toml
[dependencies]
htmx_server = { git = "https://github.com/JustBobinAround/htmx_server.git" }
```

## Example

Below is a simple example demonstrating how to use `htmx_server` to define and
serve htmx components:

```rust
use async_std::prelude::*;
use htmx_comp_macro::htmx_comp;
use htmx_server::server;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VAL1: Mutex<i32> = Mutex::new(42);
}

#[htmx_comp("/")]
fn index() -> String {
    match VAL1.lock() {
        Ok(val) => {
            Some(format!(r#"
<script src="https://unpkg.com/htmx.org@1.9.8"></script>
<!-- have a button POST a click via AJAX -->
<button hx-get="/clicked" hx-swap="innerHTML">
{}
</button>"#, val))
        },
        Err(_) => {None}
    }
}

#[htmx_comp("/clicked")]
fn click() -> Option<String>{
    match VAL1.lock() {
        Ok(mut val) => {
            *val += 1;
            Some(format!("{}", val))
        },
        Err(_) => {None}
    }
}
fn main() {
    server!("127.0.0.1:8000",[index, click]);
}
```

In this example, two htmx components, `index` and `clicked`, are defined using
the `htmx_comp` macro. The `main` function initializes the `htmx_server` on
`127.0.0.1:8000` and associates it with the defined components.

## Usage

1. Follow the example above.

2. Build and run the example

```bash
cargo run
```

3. Visit `127.0.0.1:8000`

## Change Log
**2023-11-19:**
- Started 2.0 branch to remove gotham
- Implemented basic http server
- Added routing argument to htmx_comp_macro
- Added basic shared state using lazy_static
- TODO: make lazy_static shared state feel more seamless with functions
- TODO: add maud html macro
- TODO: figure out how to better package external imports
```Rust
// your_crate/src/lib.rs

// Import the entire other_crate module1
pub mod other_crate_module1 {
    pub use other_crate::module1::*;
}

// Import only function3 from other_crate module2
pub use other_crate::module2::function3;

// Additional code for your crate...
```

## License

This project is licensed under the [MIT License](LICENSE).
