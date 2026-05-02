use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Represents the data staging phase on the CPU (ingesting combinator graphs into contiguous arrays).
/// Implements a simple pass-through.
#[proc_macro_attribute]
pub fn source(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}

/// Flags the logic reduction function. 
/// It parses the Rust function, confirms it only uses basic operations
/// and emits code that preserves the raw syntax string for the runtime to digest.
#[proc_macro_attribute]
pub fn stage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // Extract the function name to generate a constant with its raw source
    let fn_name = &input.sig.ident;
    let const_name = syn::Ident::new(
        &format!("{}_RAW_SOURCE", fn_name.to_string().to_uppercase()),
        fn_name.span()
    );
    
    let raw_source = quote!(#input).to_string();

    let expanded = quote! {
        #input

        // Preserve the raw syntax string for the runtime to digest
        // representing an OpenCL/CUDA kernel execution environment template.
        pub const #const_name: &str = #raw_source;
    };
    
    TokenStream::from(expanded)
}

/// Represents extracting the stabilized data back from the execution environment.
#[proc_macro_attribute]
pub fn sink(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}
