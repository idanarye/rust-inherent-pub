extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemImpl, ImplItem, Visibility};
use quote::ToTokens;

#[proc_macro_attribute]
pub fn inherent_pub(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input: ItemImpl = parse_macro_input!(input as ItemImpl);

    extract_pub_methods(&mut input.items);

    input.into_token_stream().into()
}

fn extract_pub_methods(items: &mut Vec<ImplItem>) {
    for mut item in items.iter_mut() {
        if let ImplItem::Method(method) = item {
            method.vis = Visibility::Inherited;
        }
    }
}
