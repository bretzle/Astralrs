extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream as Stream;
use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use syn::{
    parse, Field, GenericArgument, Ident, ItemStruct, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, PathArguments, Type,
};

enum MacroAttribute {
    Each,
}

enum FieldType {
    Regular,
    Repeated,
    Optional,
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: Stream) -> Stream {
    let source: ItemStruct = parse(input).expect("failed to parse object to item struct");

    let source_name = &source.ident;
    let builder_name = Ident::new(&format!("{}Builder", source_name), source_name.span());

    let mut builder_empty_fields = Vec::new();
    let mut builder_field_definitions = Vec::new();
    let mut builder_setters = Vec::new();
    let mut build_guards = Vec::new();
    let mut build_values = Vec::new();

    for field in &source.fields {
        builder_empty_fields.push(make_empty_field(&field));
        builder_field_definitions.push(make_builder_field(&field));
        builder_setters.push(make_setter(&field));
        let (guard, value) = make_build_guard(&field);
        build_guards.push(guard);
        build_values.push(value);
    }

    let gen = quote! {
      use std::error::Error;

      // Builder implementation
      impl #builder_name {
        // Setters for each field in the inital struct
        #(#builder_setters)*

        // Method to resolve builder to source struct
        fn build(mut self) -> Result<#source_name, Box<dyn Error>> {
          // Guard for missing fields
          #(#build_guards)*

          // Return built struct with secured values
          Ok(#source_name {
            #(#build_values)*
          })
        }
      }

      // Builder struct definition
      pub struct #builder_name {
        #(#builder_field_definitions)*
      }

      // Builder method on source struct to create a <source_name>Builder entity
      impl #source_name {
        pub fn builder() -> #builder_name {
          #builder_name {
            #(#builder_empty_fields)*
          }
        }
      }
    };

    gen.into()
}

fn make_empty_field(field: &Field) -> TokenStream {
    let name = &field.ident;
    match get_field_type(field) {
        FieldType::Repeated => quote! { #name: Vec::new(), },
        _ => quote! { #name: None, },
    }
}

fn make_builder_field(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;
    match get_field_type(field) {
        FieldType::Regular => quote! { #name: Option<#ty>, },
        _ => quote! { #name: #ty, },
    }
}

fn make_setter(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;
    match get_field_type(field) {
        FieldType::Repeated => {
            let nested_type = guess_nested_type(ty);
            let (_, repeated_setter) = get_macro_attribute(field).unwrap();
            let repeated_setter_name = Ident::new(&repeated_setter, proc_macro2::Span::call_site());
            quote! {
                fn #repeated_setter_name(mut self, #name: #nested_type) -> Self {
                  self.#name.push(#name);
                  self
                }
            }
        }
        FieldType::Optional => {
            let nested_type = guess_nested_type(ty);
            quote! {
                fn #name(mut self, #name: #nested_type) -> Self {
                  self.#name = Some(#name);
                  self
                }
            }
        }
        FieldType::Regular => quote! {
            fn #name(mut self, #name: #ty) -> Self {
              self.#name = Some(#name);
              self
            }
        },
    }
}

fn make_build_guard(field: &Field) -> (TokenStream, TokenStream) {
    let name = &field.ident;
    let guard = match get_field_type(field) {
        FieldType::Regular => quote! {
          let #name = match self.#name {
            Some(value) => value,
            None => return Err(From::from(format!("missing {} to build", stringify!(#name)))),
          };
        },
        _ => quote! { let #name = self.#name; },
    };
    (guard, quote! { #name: #name, })
}

fn get_field_type(field: &Field) -> FieldType {
    match get_macro_attribute(field) {
        Some((MacroAttribute::Each, _)) => FieldType::Repeated,
        None => match guess_nested_type(&field.ty) {
            Some(_) if get_type_name(&field.ty).expect("type should be nested") == "Option" => {
                FieldType::Optional
            }
            _ => FieldType::Regular,
        },
    }
}

fn get_macro_attribute(field: &Field) -> Option<(MacroAttribute, String)> {
    for attr in field.attrs.iter() {
        if !attr.path.is_ident("builder") {
            continue;
        }
        let meta = attr.parse_meta().expect("builder attribute to be a meta");

        let nested = match meta {
            Meta::List(MetaList { ref nested, .. }) => nested,
            _ => continue,
        };

        let first_nested = nested
            .iter()
            .next()
            .expect("no builder parameters provided");

        let builder_attr_param = match first_nested {
            NestedMeta::Meta(param_meta) => param_meta,
            _ => continue,
        };

        return match parse_attr_arg(&builder_attr_param) {
            Ok((attr, value)) => Some((attr, value)),
            Err(_) => continue,
        };
    }
    None
}

fn parse_attr_arg(meta: &Meta) -> Result<(MacroAttribute, String), Box<dyn Error>> {
    let (name, value) = match meta {
        Meta::NameValue(MetaNameValue {
            ref ident,
            lit: Lit::Str(lit_str),
            ..
        }) => (ident.to_string(), lit_str.value()),
        _ => return Err(From::from("not a name value attribute argument")),
    };
    match name.as_ref() {
        "each" => Ok((MacroAttribute::Each, value)),
        _ => Err(From::from(format!("unknown attribute argument {}", name))),
    }
}

fn guess_nested_type(ty: &Type) -> Option<Type> {
    let typepath = match ty {
        Type::Path(typepath) if typepath.qself.is_none() => typepath,
        _ => return None,
    };
    if !typepath.path.leading_colon.is_none() || typepath.path.segments.len() != 1 {
        return None;
    }
    let type_params = match &typepath.path.segments.iter().next() {
        Some(segment) => &segment.arguments,
        None => return None,
    };
    let parameters = match type_params {
        PathArguments::AngleBracketed(params) => params,
        _ => return None,
    };
    let generic_arg = match parameters.args.iter().next() {
        Some(arg) => arg,
        None => return None,
    };
    match generic_arg {
        GenericArgument::Type(ty) => Some(ty.clone()),
        _ => return None,
    }
}

fn get_type_name(ty: &Type) -> Option<Ident> {
    let path = match ty {
        Type::Path(typepath)
            if typepath.path.leading_colon.is_none() && typepath.path.segments.len() == 1 =>
        {
            &typepath.path
        }
        _ => return None,
    };
    match path.segments.iter().next() {
        Some(segment) => Some(segment.ident.clone()),
        _ => None,
    }
}
