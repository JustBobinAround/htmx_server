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
use htmx_comp_macro::htmx_comp;
use htmx_server::*;

#[htmx_comp]
fn index() {
    r#"
  <script src="https://unpkg.com/htmx.org@1.9.8"></script>
  <!-- have a button POST a click via AJAX -->
  <button hx-post="/post.clicked" hx-swap="outerHTML">
  State A
  </button>
  "#.to_string()
}

#[htmx_comp]
fn clicked() {
    "State B".to_string()
}

fn main() {
    htmx_server!("127.0.0.1:8000", [
        index,
        clicked
    ]);
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


## License

This project is licensed under the [MIT License](LICENSE).
