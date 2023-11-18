use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn htmx_comp(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_body = &input.block;

    let crate_path = quote! { ::gotham };

    let expanded = quote! {

        fn #fn_name(state: #crate_path::state::State) -> (#crate_path::state::State,
                                                          #crate_path::hyper::Response<#crate_path::hyper::Body>)
            { let mut closure: Box<dyn FnMut() -> String> = Box::new(|| {
                #fn_body
            });
            let response = #crate_path::helpers::http::response::create_response(
                &state,
                #crate_path::hyper::StatusCode::OK,
                #crate_path::mime::TEXT_HTML,
                closure()
            );
            (state, response)
        }
    };

    expanded.into()
}
