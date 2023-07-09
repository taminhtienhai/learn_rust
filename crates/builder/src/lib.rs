use proc_macro2;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Field};


fn attributes(body: syn::Data) -> Vec<syn::Field> {
    let fields = match body {
        syn::Data::Struct(struct_body) => struct_body.fields,
        _ => panic!("Should apply on a struct"),
    };

    let named = match fields {
        syn::Fields::Named(named) => named.named,
        _ => panic!(""),
    };
    
    return named.into_iter().collect::<Vec<Field>>()
}

fn to_builder_method(fields: Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    fields.into_iter().map(|Field {
        ident,
        ty,
        ..
    }| quote! {
        fn #ident(&mut self, #ident: #ty) -> &mut Self {
            self.#ident = ident;
            self
        }
    }).collect::<Vec<proc_macro2::TokenStream>>()
}

#[proc_macro_derive(Builder)]
pub fn builder_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        data,
        generics,
        ident,
        vis,
    } = parse_macro_input!(input as DeriveInput);

    let fields = attributes(data);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn generated(&self) -> bool { true }
        }
    };

    TokenStream::from(expanded)
}