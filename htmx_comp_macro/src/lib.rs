use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ItemFn, punctuated::Punctuated, FnArg, token::Comma};

#[proc_macro_attribute]
pub fn htmx_comp(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_generics = &input.sig.generics;
    let fn_inputs = &input.sig.inputs;
    let fn_body = &input.block;
    let arg_names: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat) = &*pat_type.pat {
                    if pat.ident=="state"{
                        None
                    } else {
                        Some(&pat.ident)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let crate_path = quote! { ::gotham };

    // Check if the state argument already exists in fn_inputs

    let fn_name_handler = syn::Ident::new(&format!("{}_handler", fn_name), fn_name.span());

    let expanded = quote! {
        macro_rules! #fn_name{
            ($($args:ident), *) => {
                |state| #fn_name_handler(state, $($args),*)
            };
        }

        fn #fn_name(#fn_inputs) -> String {
            #fn_body
        }
        fn #fn_name_handler(state: #crate_path::state::State, #fn_inputs) -> (#crate_path::state::State, #crate_path::hyper::Response<#crate_path::hyper::Body>)
        {
            let response = #crate_path::helpers::http::response::create_response(
                &state,
                #crate_path::hyper::StatusCode::OK,
                #crate_path::mime::TEXT_HTML,
                (|#(#arg_names),*| #fn_name(#(#arg_names),*))(#(#arg_names),*)
            );

            (state, response)
        }
    };

    expanded.into()
}
