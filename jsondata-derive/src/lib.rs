extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::*;

// TODO: handle generic-types and generic-lifetimes in struct.
// TODO: implement JsonData for enums.
// TODO: implement JsonData for tuple-struct.

#[proc_macro_derive(JsonData, attributes(json))]
#[proc_macro_error]
pub fn jsonize_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_jsonize_type(&input);
    gen.into()
}

fn impl_jsonize_type(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    match &input.data {
        Data::Struct(ast) => match &ast.fields {
            Fields::Named(fields) => {
                let mut ts = from_type_to_json(name, fields);
                ts.extend(from_json_to_type(name, fields));
                ts
            }
            _ => abort_call_site!("jsondata only supports named fields"),
        },
        _ => abort_call_site!("jsondata only supports named structs"),
    }
}

fn from_type_to_json(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let mut token_builder = quote! {};
    for field in fields.named.iter() {
        let field_name = match &field.ident {
            Some(field_name) => field_name,
            None => continue,
        };
        let key = field_name.to_string().to_lowercase();
        let prop = quote! {
            let v = match value.#field_name.try_into() {
                Ok(v) => Ok(v),
                Err(err) => Err(::jsondata::Error::InvalidType(#key.to_string())),
            }?;
            props.push(::jsondata::Property::new(#key, v));
        };
        token_builder.extend(prop);
    }
    quote! {
        impl ::std::convert::TryFrom<#name> for ::jsondata::Json {
            type Error = ::jsondata::Error;

            fn try_from(value: #name) -> Result<::jsondata::Json, Self::Error> {
                let mut props: Vec<::jsondata::Property> = vec![];
                #token_builder;
                Ok(::jsondata::Json::new(props))
            }
        }
    }
}

fn from_json_to_type(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let mut token_builder = quote! {};
    for field in fields.named.iter() {
        let field_name = match &field.ident {
            Some(field_name) => field_name,
            None => continue,
        };
        let key = field_name.to_string().to_lowercase();
        token_builder.extend(quote! {
            #field_name: match value.get(&("/".to_string() + #key))?.try_into() {
                Ok(v) => Ok(v),
                Err(err) => Err(::jsondata::Error::InvalidType(#key.to_string())),
            }?,
        });
    }
    quote! {
        impl ::std::convert::TryFrom<::jsondata::Json> for #name {
            type Error = ::jsondata::Error;

            fn try_from(value: ::jsondata::Json) -> Result<#name, Self::Error> {
                Ok(#name {
                    #token_builder
                })
            }
        }
    }
}
