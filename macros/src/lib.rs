extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Ident, ItemStruct};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct = parse(input).expect("failed to parse object to item struct");
    let struct_name = &item.ident;
    let builder_struct_name = Ident::new(&format!("{}Builder", struct_name), struct_name.span());

    let mut builder_struct_empty = Vec::new();
    let mut builder_struct_fields = Vec::new();

    for field in &item.fields {
        let ident = &field.ident;
        let ty = &field.ty;

        builder_struct_fields.push(quote! {
            #ident: Option<#ty>,
        });
        builder_struct_empty.push(quote! {
            #ident: None,
        });
    }

    let gen = quote! {
        impl #struct_name {
            pub fn builder() -> #builder_struct_name {
                #builder_struct_name {
                    #(#builder_struct_empty)*
                }
            }
        }

        pub struct #builder_struct_name {
            #(#builder_struct_fields)*
        }
    };

    gen.into()
}
