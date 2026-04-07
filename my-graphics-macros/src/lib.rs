use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Expr, Lit, Meta};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

struct AttrConfig {
    title: String,
    width: f32,
    height: f32,
}

impl Parse for AttrConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut title = "My App".to_string();
        let mut width = 800.0f32;
        let mut height = 600.0f32;

        let pairs: Punctuated<Meta, Token![,]> = input.parse_terminated(Meta::parse, Token![,])?;
        for meta in pairs {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("title") {
                    if let Expr::Lit(el) = nv.value {
                        if let Lit::Str(ls) = el.lit {
                            title = ls.value();
                        }
                    }
                } else if nv.path.is_ident("width") {
                    if let Expr::Lit(el) = nv.value {
                        if let Lit::Float(lf) = &el.lit {
                            width = lf.base10_parse::<f32>()?;
                        } else if let Lit::Int(li) = &el.lit {
                            width = li.base10_parse::<f32>()?;
                        }
                    }
                } else if nv.path.is_ident("height") {
                    if let Expr::Lit(el) = nv.value {
                        if let Lit::Float(lf) = &el.lit {
                            height = lf.base10_parse::<f32>()?;
                        } else if let Lit::Int(li) = &el.lit {
                            height = li.base10_parse::<f32>()?;
                        }
                    }
                }
            }
        }

        Ok(AttrConfig { title, width, height })
    }
}

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let config = if attr.is_empty() {
        AttrConfig { title: "My App".to_string(), width: 800.0, height: 600.0 }
    } else {
        parse_macro_input!(attr as AttrConfig)
    };

    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;

    let title = config.title;
    let width = config.width;
    let height = config.height;

    let expanded = quote! {
        fn main() {
            my_graphics::run(#title, #width, #height, move |mut win| {
                #block
            });
        }
    };

    TokenStream::from(expanded)
}
