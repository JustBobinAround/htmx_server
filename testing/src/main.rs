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
