# htmx_server

## Overview

`htmx_server` is a Rust crate that simplifies server-side of
[htmx](https://htmx.org/) components, not that it was that hard anyway. This crate is simply meant to 
ship htmx and rust stack apps in the most minimal code possible. This crate is still in very early development.
This crate ships with Maud's HTML! macro along with a basic http server, router, and state management 
system to handle the GET and POST requests.

## Getting Started

To use `htmx_server` in your Rust project, add it as a dependency in your
`Cargo.toml` file:

```toml
[dependencies]
htmx_server = { git = "https://github.com/JustBobinAround/htmx_server.git" }
async-std = "1.10"
maud = "0.25.0"
```

## Example

Below is a simple example demonstrating how to use `htmx_server` to define and
serve htmx components:

```rust
use htmx_server::prelude::*;

lazy_static! {
    static ref VAL1: Global<i32> = Global::new(42);
}

#[htmx_comp("/")]
fn index() -> Option<String> {
    global!(VAL1);
    let mut response: Option<String> = None;

    lock_globals!(response, val1;{
        html!{
            script src="https://unpkg.com/htmx.org@1.9.8" {}
            button hx-get="/clicked" hx-swap="innerHTML" {(val1)}
        }
    });

    response

}

#[htmx_comp("/clicked")]
fn click() -> Option<String>{
    let mut response: Option<String> = None;
    global!(VAL1);

    lock_globals!(response, val1;{
        *val1 += 1;
        html!({(val1)})
    });

    response
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

## HTML!
Visit [Maud](https://maud.lambda.xyz/) to view the html! syntax

## Routing
Routing is handled by the `htmx_comp` proc macro attribute:
```rust
#[htmx_comp("/your_route")]
fn some_function() -> Option<String>{
    let mut response: Option<String> = None;
    global!(SOME_GLOBAL);

    lock_globals!(response, some_global;{
        *some_global += 1;
        html!({(some_global)})
    });

    response
}

```
## Http Server
Routing functions are then called as closures in the `server!` macro:
```rust
fn main() {
        server!("127.0.0.1:8000",[some_function]);
}
```

## State Management
State management is handled with the `Global` type via `lazy_static!`:
```rust
lazy_static! {
    static ref SOME_GLOBAL: Global<i32> = Global::new(42);
}
```
Globals can then be accessed in a `htmx_comp` function. Globals are declared
via the `global!` macro, and are the accessable via the `lock_globals` macro.
Notice how `SOME_GOBAL` is accessed as `some_global`. This is not a typo, this
macro automatically creates a new locked variable called `some_global` to avoid
shadowing a static reference. Any mutable state must be handled in the `lock_globals`
code block. Non blocking reads are in the works currentlly...
```rust
#[htmx_comp("/your_route")]
fn some_function() -> Option<String>{
    let mut response: Option<String> = None;
    global!(SOME_GLOBAL);

    lock_globals!(response, some_global;{
        *some_global += 1;
        html!({(some_global)})
    });

    response
}
```

## Change Log
**2023-11-19:**
- Started 2.0 branch to remove gotham
- Implemented basic http server
- Added routing argument to htmx_comp_macro
- Added basic shared state using lazy_static
- TODO: make lazy_static shared state feel more seamless with functions
- TODO: add maud html macro
- TODO: figure out how to better package external imports

**2023-11-20:**
- Finished TODOs from **2023-11-20**
- Added an awesome macro to handle nested mutex locks easily
- TODO: Change Mutex to RwLock for non blocking reads

## Special Thanks
Thanks to [Maud](https://github.com/lambda-fairy/maud) for making an awesome macro.
This library wouldn't feel right without it.

## License

This project is licensed under the [MIT License](LICENSE).
