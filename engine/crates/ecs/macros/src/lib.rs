use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, Ident, LitStr, Path, Result
};

use uengine_macro::Manifest;

fn crate_path() -> Path {
    Manifest::default().get_path("uengine_ecs")
}

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let crate_path = crate_path();

    let attrs = match parse_component_attr(&ast) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error().into(),
    };

    let storage = storage_path(&crate_path, attrs.storage);

    let struct_name = &ast.ident;

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: 'static });

    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_path::component::Component for #struct_name #type_generics #where_clause {
            const STORAGE_POLICY: #crate_path::component::StoragePolicy = #storage;
        }
    }.into()
}

#[derive(Clone, Copy)]
enum StoragePolicy {
    Dense,
    Sparse,
}

struct Attrs {
    storage: StoragePolicy
}

const COMPONENT: &str = "component";
const STORAGE: &str = "storage";

const DENSE: &str = "dense";
const SPARSE: &str = "sparse";

fn parse_component_attr(ast: &DeriveInput) -> Result<Attrs> {
    let mut attrs = Attrs {
        storage: StoragePolicy::Dense
    };

    for attr in ast.attrs.iter() {
        if attr.path().is_ident(COMPONENT) {
            attr.parse_nested_meta(|nested| {
                if nested.path.is_ident(STORAGE) {
                    attrs.storage = match nested.value()?.parse::<LitStr>()?.value() {
                        s if s == DENSE => StoragePolicy::Dense,
                        s if s == SPARSE => StoragePolicy::Sparse,
                        s => {
                            return Err(nested.error(format!(
                                "Invalid storage type `{s}`, expected '{DENSE}' or '{SPARSE}'.",
                            )));
                        }
                    };
                    Ok(())
                } else {
                    Err(nested.error("Unsupported attribute"))
                }
            })?;
        }
    }

    Ok(attrs)
}

fn storage_path(crate_path: &Path, policy: StoragePolicy) -> TokenStream2 {
    let storage_type = match policy {
        StoragePolicy::Dense => Ident::new("Dense", Span::call_site()),
        StoragePolicy::Sparse => Ident::new("Sparse", Span::call_site()),
    };

    quote! { #crate_path::component::StoragePolicy::#storage_type }
}
