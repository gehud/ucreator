extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse::Parser, parse_macro_input, punctuated::Punctuated, Error, ItemStruct, Ident, Path, Token};

#[proc_macro_attribute]
pub fn system(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let args = Punctuated::<Ident, Token![,]>::parse_terminated.parse(args);

    match args {
        Ok(arg) => {
            if arg.len() == 0 {
                quote! {
                    #[derive(Default)]
                    #item
                }.into()
            } else if arg.len() != 1 {
                Error::new_spanned(arg, "wrong arguments number").to_compile_error().into()
            } else {
                if arg.first().unwrap() != "Default" {
                    Error::new_spanned(arg, "'Default' expected").to_compile_error().into()
                } else {
                    quote! {
                        #item
                    }.into()
                }
            }
        },
        Err(err) => {
            err.to_compile_error().into()
        }
    }
}

#[proc_macro_attribute]
pub fn group(arg: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let arg = parse_macro_input!(arg as Path);

    let ident = &item.ident;

    quote! {
        #item

        impl ::uengine::ecs::SystemGroup for #ident {
            type System = #ident;
            type Group = #arg;
        }
    }.into()
}
