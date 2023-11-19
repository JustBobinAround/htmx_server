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
