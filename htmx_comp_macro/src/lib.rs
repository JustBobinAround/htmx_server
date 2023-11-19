use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,Ident ,ItemFn,NestedMeta, Lit, AttributeArgs};

#[proc_macro_attribute]
pub fn htmx_comp(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let arg_value_get = match &args[..] {
        [NestedMeta::Lit(Lit::Str(ref value))] => value.value(),
        _ => panic!("The attribute should have exactly one string argument."),
    };
    
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_body = &input.block;
    if fn_inputs.len()>0 {
        panic!("A Function with the htmx_comp attribute should contain 0 arguments");
    }
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

#[proc_macro]
pub fn global(input: TokenStream) -> TokenStream {
    let input_ident = parse_macro_input!(input as Ident);

    let lowercase_name = input_ident.to_string().to_lowercase();
    let lowercase_ident = Ident::new(&lowercase_name, input_ident.span());

    let expanded = quote! {
        let #lowercase_ident = #input_ident.clone();
    };

    TokenStream::from(expanded)
}

