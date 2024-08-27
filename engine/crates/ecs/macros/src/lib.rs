extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn component(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let ident = &item.ident;

    quote! {
        #item

        impl ::uengine_ecs::Component for #ident {
            const STORAGE_TYPE: ::uengine_ecs::StorageType = ::uengine_ecs::StorageType::Archetype;
        }
    }.into()
}
