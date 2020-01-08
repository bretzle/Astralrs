extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use std::error::Error;
use syn::{parse, Ident, ItemStruct, Type};

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
        let type_ = match get_option_type(&field.ty) {
            Ok(t) => t,
            Err(_) => field.ty.clone(),
        };
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
        let type_ = match get_option_type(&field.ty) {
            Ok(t) => t,
            Err(_) => field.ty.clone()
        };

        setters.push(quote! {
            fn #name(mut self, #name: #type_) -> Self {
                self.#name = Some(#name);
                self
            }
        })
    }

    let build_method = make_build_method(source);

    quote! {
        impl #builder_name {
            #(#setters)*

            #build_method
        }
    }
}

fn make_build_method(source: &ItemStruct) -> proc_macro2::TokenStream {
    let mut checks = Vec::new();
    let mut values = Vec::new();
    
    let source_name = &source.ident;

    for field in &source.fields {
        let name = &field.ident;
    
        values.push(quote! { #name: #name, });

        checks.push(
            match get_option_type(&field.ty) {
                Ok(_) => quote!{ let #name = self.#name; },
                Err(_) => quote!{
                    let #name = match self.#name {
                        Some(val) => val,
                        None => return Err(From::from(format!("missing {} to build {}", stringify!(#name), stringify!(#source_name)))),
                    };
                },
            }
        );
    }

    quote! {
        fn build(mut self) -> Result<#source_name, Box<dyn Error>> {
            #(#checks)*
            Ok(#source_name { #(#values)* })
        }
    }
}

fn get_option_type(type_: &Type) -> Result<Type, Box<dyn Error>> {
    match type_ {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            let type_params = match &typepath.path.segments.iter().next() {
                Some(segment) => &segment.arguments,
                None => return Err(From::from("failed to get first segment arg"))
            };

            let parameters = match type_params {
                syn::PathArguments::AngleBracketed(params) => params,
                _ => return Err(From::from("no angle brackets"))
            };

            let generic_arg = match parameters.args.iter().next() {
                Some(arg) => arg,
                None => return Err(From::from("failed to get generic argument"))
            };

            match generic_arg {
                syn::GenericArgument::Type(ty) => Ok(ty.clone()),
                _ => return Err(From::from("impossible to extract type"))
            }
        }
        _ => Err(From::from("not an option")),
    }
}

fn path_is_option(path: &syn::Path) -> bool {
    if path.leading_colon.is_none() && path.segments.len() == 1 {
        return match path.segments.iter().next() {
            Some(segment) => &segment.ident == "Option",
            None => false
        };
    }
    return false;
}
