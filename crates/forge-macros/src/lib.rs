use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, ReturnType, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemFn = parse_macro_input!(item);

    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(input.sig.fn_token, "#[main] Requires an async function")
            .to_compile_error()
            .into();
    }

    if !input.sig.inputs.is_empty() {
        return syn::Error::new_spanned(&input.sig.inputs, "#[main] Does not accept parameters")
            .to_compile_error()
            .into();
    }

    if !matches!(input.sig.output, ReturnType::Default) {
        return syn::Error::new_spanned(&input.sig.output, "#[main] Must not return a value")
            .to_compile_error()
            .into();
    }

    let original_name: &syn::Ident = &input.sig.ident;
    let async_name: syn::Ident = format_ident!("__forge_async_{}", original_name);

    input.sig.ident = async_name.clone();

    let attrs: &Vec<syn::Attribute> = &input.attrs;
    let vis: &syn::Visibility = &input.vis;

    let expanded = quote! {
        #(#attrs)*
        #vis #input

        fn main() {
            let threads = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(8) * 12;

            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(threads)
                .enable_all()
                .build()
                .expect("Forge runtime build failed")
                .block_on(#async_name());
        }
    };

    expanded.into()
}
