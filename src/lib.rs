extern crate buddy;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemStatic};

#[proc_macro_attribute]
pub fn define_simple_allocator(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStatic);
    let size = parse_macro_input!(attr as Expr);
    let name = &input.ident;

    let expanded = quote! {
        #[global_allocator]
        static ALLOC: simple::SimpleAllocator = simple::SimpleAllocator::new(unsafe { &mut #name });
        static mut #name: [u8; #size] = [0; #size];
    };

    TokenStream::from(expanded)
}
