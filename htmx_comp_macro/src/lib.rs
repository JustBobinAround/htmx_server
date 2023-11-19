use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ItemFn,Meta, NestedMeta, Lit, AttributeArgs, punctuated::Punctuated, FnArg, token::Comma};

#[proc_macro_attribute]
pub fn htmx_comp(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let arg_value_get = match &args[..] {
        [NestedMeta::Lit(Lit::Str(ref value))] => value.value(),
        _ => panic!("The attribute should have exactly one string argument."),
    };
    
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_generics = &input.sig.generics;
    let fn_inputs = &input.sig.inputs;
    let fn_body = &input.block;
    if fn_inputs.len()>0 {
        panic!("A Function with the htmx_comp attribute should contain 0 arguments");
    }


    let async_std = quote! { ::async_std};

    // Check if the state argument already exists in fn_inputs

    let fn_name_handler = syn::Ident::new(&format!("{}_handler", fn_name), fn_name.span());

    let expanded = quote! {
        fn #fn_name(url: &str) -> Option<String> {
            let a = #arg_value_get;
            if url==a {
                {#fn_body}
            } else {
                None
            }
        }
    };

    expanded.into()
}

