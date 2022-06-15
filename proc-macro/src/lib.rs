use proc_macro::{self, TokenStream};
use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, AttributeArgs, Field, FieldsNamed, ItemStruct};

use darling::FromMeta;
use std::{any::Any, collections::HashMap, path::Path};
use syn::spanned::Spanned;
use toml::{value::Table, Value};

struct ValueToTokens {
    value: Value,
    ty: Option<syn::Type>,
}

impl ToTokens for ValueToTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.value {
            Value::String(string) => {
                string.to_tokens(tokens);
            }
            Value::Table(map) => {
                if let Some(ty) = &self.ty {
                    let mut fields = Vec::new();

                    // todo: support multiple levels of structs deep
                    for (k, v) in map {
                        let val = ValueToTokens {
                            value: v.clone(),
                            ty: None,
                        };

                        let key = syn::Ident::new(&k, k.span());

                        fields.push(quote! { #key: #val });
                    }

                    quote! { #ty { #(#fields,)* } }.to_tokens(tokens);
                }
            }
            _ => {}
        }
    }
}

enum File {
    Toml(String),
}

impl File {
    pub fn from_file_path<S: Into<String>>(file_path: S) -> Option<Self> {
        let file_path = file_path.into();
        match Path::new(&file_path).extension().unwrap().to_str() {
            Some("toml") => Some(Self::Toml(std::fs::read_to_string(file_path).unwrap())),
            _ => None,
        }
    }

    fn parse(&self, type_map: HashMap<String, syn::Type>) -> HashMap<String, Box<dyn ToTokens>> {
        return match &*self {
            Self::Toml(contents) => {
                let mut to_return: HashMap<String, Box<dyn ToTokens>> = HashMap::new();

                let value = contents.parse::<Value>().unwrap();
                let table = value.try_into::<Table>().unwrap();

                for key in table.keys() {
                    let value = table.get(key).unwrap().clone();
                    let ty = type_map.get(key).map_or(None, |f| Some(f.clone()));

                    to_return.insert(key.to_owned(), Box::new(ValueToTokens { value, ty }));
                }

                to_return
            }
        };
    }
}

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
