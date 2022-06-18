use proc_macro::{self, TokenStream};
use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, AttributeArgs, Field, FieldsNamed, ItemStruct};

use darling::FromMeta;
use std::{any::Any, collections::HashMap, path::Path};
use syn::spanned::Spanned;
use toml::{value::Table, Value};

pub(crate) struct ValueToTokens {
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
            Value::Array(arr) => {
                let arr: Vec<ValueToTokens> = arr
                    .iter()
                    .map(|x| ValueToTokens {
                        value: x.clone(),
                        ty: None,
                    })
                    .collect();
                quote! { &[#(#arr,)*] }.to_tokens(tokens);
            }
            _ => {}
        }
    }
}

pub(crate) enum File {
    Toml(String),
}

impl File {
    pub(crate) fn from_file_path<S: Into<String>>(file_path: S) -> Option<Self> {
        let file_path = file_path.into();
        match Path::new(&file_path).extension().unwrap().to_str() {
            Some("toml") => Some(Self::Toml(std::fs::read_to_string(file_path).unwrap())),
            _ => None,
        }
    }

    pub(crate) fn parse(
        &self,
        type_map: HashMap<String, syn::Type>,
    ) -> HashMap<String, Box<dyn ToTokens>> {
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
pub(crate) struct Arguments {
    pub(crate) path: String,
}
