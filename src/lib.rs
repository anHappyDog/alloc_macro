#![no_std]

extern crate proc_macro;
extern crate buddy;


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStatic};

#[proc_macro_attribute]
pub fn define_allocator(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStatic);
    let size = attr.to_string().parse::<usize>().unwrap();
    let name = &input.ident;

    let expanded = quote! {
        #[global_allocator]
        static #name: simple::SimpleAllocator =simple::SimpleAllocator::new(unsafe { &mut #name });

        static mut #name: [u8; #size] = [0; #size];
    };

    TokenStream::from(expanded)
}