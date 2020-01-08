extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Ident, ItemStruct};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct = parse(input).expect("failed to parse object to item struct");

    let name = &item.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());

    let builder_struct = make_builder_struct(&item, &builder_name);
    let struct_trait = make_struct_trait(&item, &builder_name);
    let setters_trait = make_setters_trait(&item, &builder_name);

    let gen = quote! {
        use std::error::Error;

        #struct_trait

        #builder_struct

        #setters_trait
    };

    gen.into()
}

fn make_builder_struct(source: &ItemStruct, builder_name: &Ident) -> proc_macro2::TokenStream {
    let mut field_tokens = Vec::new();

    for field in &source.fields {
        let name = &field.ident;
        let type_ = &field.ty;
        field_tokens.push(quote! {
            #name: Option<#type_>,
        });
    }

    quote! {
        pub struct #builder_name {
            #(#field_tokens)*
        }
    }
}

fn make_struct_trait(source: &ItemStruct, builder_name: &Ident) -> proc_macro2::TokenStream {
    let mut fields = Vec::new();
    let source_name = &source.ident;
    
    for field in &source.fields {
        let name = &field.ident;
        fields.push(quote! {
            #name: None,
        });
    }

    quote! {
        impl #source_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#fields)*
                }
            }
        }
    }
}

fn make_setters_trait(source: &ItemStruct, builder_name: &Ident) -> proc_macro2::TokenStream {
    let mut setters = Vec::new();

    for field in &source.fields {
        let name = &field.ident;
        let type_ = &field.ty;

        setters.push(quote! {
            fn #name(mut self, #name: #type_) -> Self {
                self.#name = Some(#name);
                self
            }
        })
    }

    let build_method = make_build_method(source, builder_name);

    quote! {
        impl #builder_name {
            #(#setters)*

            #build_method
        }
    }
}

fn make_build_method(source: &ItemStruct, builder_name: &Ident) -> proc_macro2::TokenStream {
    let source_name = &source.ident;
    let mut checks = Vec::new();
    let mut values = Vec::new();

    for field in &source.fields {
        let name = &field.ident;
        checks.push(quote! {
            #name: Some(#name),
        });
        values.push(quote! {
            #name: #name,
        });
    }

    quote! {
        pub fn build(mut self) -> Result<#source_name, Box<dyn Error>> {
            match self {
                #builder_name { #(#checks)* } => Ok(#source_name { #(#values)* }),
                _ => Err(From::from(format!(
                    "missing some fields to build{}", stringify!(#source_name)
                )))
            }
        }
    }
}
