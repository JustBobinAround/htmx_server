#[macro_export]
macro_rules! add_routes {
    ($route:ident, $($closure:expr),*) => {
        $(
            
            let r = stringify!($closure);
            let idx = match r.find('!') {
                Some(idx) => idx,
                None => 0
            };

            if &r[..idx]=="index" {
                $route.get("/").to($closure);
            }else{
                let r_get = format!("/get.{}",r[..idx].to_string());
                let r_post = format!("/post.{}",r[..idx].to_string());
                $route.get(&r_get).to($closure);
                $route.post(&r_post).to($closure);
            }
        )*
    };
}

#[macro_export]
macro_rules! better_router {
    ($($closure:expr), *) => {
        build_simple_router(|route| {
            add_routes!(route,$($closure),*);
        })
    };
}

#[macro_export]
macro_rules!  htmx_server{

    ($addr:expr, [$($closure:expr), *]) => {
        use htmx_comp_macro::htmx_comp;
        use gotham::router::Router;
        use gotham::state::State;
        use gotham::router::builder::*;
        use gotham::helpers::http::response::create_response;
        use gotham::mime;
        use gotham::hyper::{ Response, StatusCode, Body };
        fn router() -> Router {
            better_router!(
                $($closure), *
            )
        }
        println!("Listening for requests at http://{}", $addr);
        let _ = gotham::start($addr, router());
    };
}
