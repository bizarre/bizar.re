use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, token, Attribute, AttributeArgs, DataEnum, DataUnion, Field,
    FieldsNamed, FieldsUnnamed, Ident, ItemStatic, ItemStruct, Meta, Token,
};

use darling::FromMeta;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use toml::value::Table;
use toml::Value;

#[derive(Debug, FromMeta)]
struct Arguments {
    path: String,
}

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

    let foreign_fields: HashMap<String, String> =
        match Path::new(&path).extension().unwrap().to_str() {
            Some("toml") => {
                let file = std::fs::read_to_string(path).unwrap();
                let mut to_return = HashMap::new();

                let value = file.parse::<Value>().unwrap();
                let table = value.try_into::<Table>().unwrap();

                for key in table.keys() {
                    to_return.insert(key.to_owned(), table[key].as_str().unwrap().to_string());
                }

                to_return
            }
            _ => {
                panic!("Unsupported configuration file type!")
            }
        };

    let mut mapped_fields = HashMap::new();

    match fields.clone() {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            for field in named.iter() {
                mapped_fields.insert(field.ident.as_ref(), field.ty.clone());
            }
        }
        _ => panic!("Invalid!"),
    }

    let field_vis = fields.iter().map(|field| &field.vis);
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    let field_name_cloned = field_name.clone();
    let fields = field_name_cloned.map(|name| {
        let mapped = foreign_fields.get(&name.as_ref().unwrap().to_string());
        quote! {
            #name: #mapped
        }
    });

    TokenStream::from(quote! {
        #(#attrs)* #vis struct #ident {
           #(#field_vis #field_name: #field_type,)*
        }

        pub static instance: #ident = #ident {
            #(#fields,)*
        };
    })
}
