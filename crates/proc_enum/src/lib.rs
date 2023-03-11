extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn my_derive(_metadata: TokenStream, _input: TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(_input);

    println!("{:?}", input);

    proc_macro::TokenStream::from(quote!{struct H { a: String }})
}

fn extract_enum(item: Item) -> syn::ItemEnum {
    let Item::Enum(inner) = item else {
        panic!("not an enum");
    };

    return inner;
}

#[proc_macro_attribute]
pub fn proc_struct_enum(_metadata: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Item);
    let enu = extract_enum(input);

    proc_macro::TokenStream::from(quote!{struct H { a: String }})
}