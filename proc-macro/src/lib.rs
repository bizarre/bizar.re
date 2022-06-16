use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, AttributeArgs, FieldsNamed, ItemStruct};

use darling::FromMeta;
use std::collections::HashMap;
use syn::spanned::Spanned;

use quote::quote;
mod token;

use token::*;

#[proc_macro_attribute]
pub fn inject_from_file(_arguments: TokenStream, input: TokenStream) -> TokenStream {
    let ItemStruct {
        attrs,
        vis,
        ident,
        fields,
        ..
    } = parse_macro_input!(input);

    // read attributes into list of meta
    let arguments = parse_macro_input!(_arguments as AttributeArgs);
    let Arguments { path } = match Arguments::from_list(&arguments) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let mut mapped_fields = HashMap::new();

    match fields.clone() {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            for field in named.iter() {
                mapped_fields.insert(field.ident.clone(), field.ty.clone());
            }
        }
        _ => panic!("Invalid!"),
    }

    let file = File::from_file_path(path).expect("Unsupported file type.");
    let foreign_fields = file.parse(
        mapped_fields
            .iter()
            .map(|(k, v)| (k.clone().unwrap().to_string(), v.clone()))
            .collect(),
    );

    let mut constructor_fields = Vec::new();
    let mut struct_fields = Vec::new();

    for field in fields.iter() {
        let syn::Field { vis, ident, ty, .. } = field;
        struct_fields.push(quote! {
            #vis #ident: #ty
        })
    }

    for (k, v) in foreign_fields {
        let key = syn::Ident::new(&k, k.span());
        match file {
            File::Toml(_) => {
                constructor_fields.push(quote! { #key: #v });
            }
        }
    }

    TokenStream::from(quote! {
        #(#attrs)* #vis struct #ident {
           #(#struct_fields,)*
        }

        pub static instance: #ident = #ident {
            #(#constructor_fields,)*
        };
    })
}
