use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;

    let expanded = quote! {
        fn main() {
            my_graphics::run(move |mut win| {
                #block
            });
        }
    };

    TokenStream::from(expanded)
}
