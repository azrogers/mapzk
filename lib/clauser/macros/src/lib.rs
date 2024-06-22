#![feature(log_syntax)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate synstructure;

mod error;

use std::str::FromStr;

use proc_macro2::Span;

use error::MacroError;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, spanned::Spanned, DataStruct, DeriveInput, Field, Fields, GenericArgument,
    Ident, Macro, Type,
};

struct StructFields<'a> {
    normal: Vec<&'a Field>,
    duplicate: Vec<&'a Field>,
}

struct StructFieldsDeclarations<'a>(&'a StructFields<'a>);

impl<'a> ToTokens for StructFieldsDeclarations<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.duplicate.iter().for_each(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_type = &f.ty;
            tokens.append_all(quote! {
                let mut #field_name: #field_type = Vec::new();
            })
        });

        self.0.normal.iter().for_each(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_type = &f.ty;
            tokens.append_all(quote! {
                let mut #field_name: Option<#field_type> = None;
            })
        });
    }
}

// (fields, as_str)
struct StructFieldsNames<'a>(&'a StructFields<'a>, bool);

impl<'a> ToTokens for StructFieldsNames<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names: Vec<TokenStream> = self
            .0
            .normal
            .iter()
            .chain(self.0.duplicate.iter())
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                match self.1 {
                    true => ident.to_string().into_token_stream(),
                    false => ident.into_token_stream(),
                }
            })
            .collect();

        tokens.append_separated(names, quote! { , });
    }
}

struct StructFieldsOptionChecks<'a>(&'a StructFields<'a>);

impl<'a> ToTokens for StructFieldsOptionChecks<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0
            .normal
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .map(|i| {
                let istr = i.to_string();
                quote! {
                    let #i = #i.ok_or(serde::de::Error::missing_field(#istr))?;
                }
            })
            .for_each(|t| tokens.append_all(t));
    }
}

impl<'a> StructFields<'a> {
    fn to_declarations(&'a self) -> StructFieldsDeclarations<'a> {
        StructFieldsDeclarations(self)
    }

    fn to_names(&'a self, as_str: bool) -> StructFieldsNames<'a> {
        StructFieldsNames(self, as_str)
    }

    fn to_option_checks(&'a self) -> StructFieldsOptionChecks<'a> {
        StructFieldsOptionChecks(self)
    }
}

fn unwrap_generic_type(t: &Type) -> Result<Type, MacroError> {
    match t {
        Type::Path(p) => {
            let p = &p.path;
            let last = p
                .segments
                .last()
                .ok_or(MacroError::Message("no type path found", p.span()))?;
            let args = match &last.arguments {
                syn::PathArguments::None => Err(MacroError::Message(
                    "can't get generic type of a type without arguments",
                    last.span(),
                )),
                syn::PathArguments::AngleBracketed(params) => Ok(&params.args),
                syn::PathArguments::Parenthesized(params) => Err(MacroError::Message(
                    "don't know how to handle paranthesized generic arguments",
                    last.span(),
                )),
            }?;

            for a in args {
                if let GenericArgument::Type(gt) = a {
                    return Ok(gt.clone());
                }
            }

            Err(MacroError::Message(
                "unable to find generic type parameter",
                t.span(),
            ))
        }
        _ => Err(MacroError::Message(
            "only Type::Path is supported",
            t.span(),
        )),
    }
}

fn field_has_attr_for_duplicate(field: &Field) -> Result<bool, MacroError> {
    for a in &field.attrs {
        match &a.meta {
            syn::Meta::Path(p) => {
                let ident = &p
                    .segments
                    .last()
                    .ok_or(MacroError::Message(
                        "couldn't get path of attribute",
                        p.span(),
                    ))?
                    .ident;

                if ident == "from_duplicate_key" {
                    return Ok(true);
                }

                ()
            }
            _ => (),
        }
    }

    Ok(false)
}

fn discern_fields<'a>(s: &'a mut DataStruct) -> Result<Option<StructFields<'a>>, MacroError> {
    let mut normal_fields: Vec<&'a Field> = Vec::new();
    let mut duplicate_fields: Vec<&'a Field> = Vec::new();

    let fields = &mut s.fields;

    match fields {
        Fields::Named(named) => {
            for f in &named.named {
                match field_has_attr_for_duplicate(f)? {
                    true => duplicate_fields.push(f),
                    false => normal_fields.push(f),
                }
            }

            Ok(Some(StructFields {
                normal: normal_fields,
                duplicate: duplicate_fields,
            }))
        }
        _ => Ok(None),
    }
}

fn field_names_ident_for_struct(name: &TokenStream) -> Result<TokenStream, MacroError> {
    let tokens = TokenStream::from_str(&format!("_Cl_{}_FIELDS", name.to_string()))?;
    Ok(tokens)
}

fn visitor_name(name: &TokenStream) -> Result<TokenStream, MacroError> {
    Ok(TokenStream::from_str(&format!("{}Visitor", name))?.into())
}

fn visitor_for_struct(name: &TokenStream, s: &mut DataStruct) -> Result<TokenStream, MacroError> {
    let visitor = visitor_name(name)?;
    let struct_fields = discern_fields(s)?.ok_or(MacroError::Message(
        "from_duplicate_key only supported on named fields",
        name.span(),
    ))?;

    let name_str = name.to_string();

    let declarations = struct_fields.to_declarations().to_token_stream();
    let len = struct_fields.normal.len() + struct_fields.duplicate.len();
    let names_const_label = field_names_ident_for_struct(name)?;
    let names_contents = struct_fields.to_names(true).to_token_stream();
    let names_def = quote! {
        impl #name {
            const #names_const_label: [&'static str; #len] = [#names_contents];
        }
    };

    let field_cons = struct_fields.to_names(false).to_token_stream();
    let checks = struct_fields.to_option_checks().to_token_stream();

    let StructFields { normal, duplicate } = struct_fields;

    let normal_match_arms: Vec<TokenStream> = normal
        .iter()
        .map(|f| (f.ident.as_ref().unwrap(), &f.ty))
        .map(|(s, t)| {
            let sstr = s.to_string();
            quote! {
                #sstr => {
                    let val = map.next_value::<#t>()?;
                    #s = Some(val);
                    Ok(())
                }
            }
        })
        .collect();

    let duplicate_match_arms: Vec<TokenStream> = (duplicate
        .iter()
        .map(|f| (f.ident.as_ref().unwrap(), &f.ty))
        .map(|(s, t)| Ok((s, unwrap_generic_type(t)?)))
        .collect::<Result<Vec<(&Ident, Type)>, MacroError>>())?
    .into_iter()
    .map(|(s, t)| {
        let sstr = s.to_string();
        quote! {
            #sstr => {
                let val = map.next_value::<#t>()?;
                #s.push(val);
                Ok(())
            }
        }
    })
    .collect();

    Ok(quote! {
        #names_def

        struct #visitor;
        impl<'de> serde::de::Visitor<'de> for #visitor
        {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a {}", #name_str)
            }

            fn visit_map<A>(self, mut map: A) -> Result<#name, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #declarations

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        #(#normal_match_arms),*
                        #(#duplicate_match_arms),*
                        _ => Err(serde::de::Error::unknown_field(key, &#name::#names_const_label)),
                    }?;
                }

                #checks

                Ok(#name {
                    #field_cons
                 })
            }
        }
    })
}

fn deserializer_for_struct(name: &TokenStream) -> Result<TokenStream, MacroError> {
    let visitor = visitor_name(name)?;
    Ok(quote! {
        impl<'de> serde::de::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_map(#visitor)
            }
        }
    }
    .into())
}

fn duplicate_keys_impl(input: proc_macro::TokenStream) -> Result<TokenStream, MacroError> {
    let original: TokenStream = input.clone().into();
    let mut item: DeriveInput = syn::parse(input).unwrap();

    let name = item.ident.to_token_stream();

    let visitor = match &mut item.data {
        syn::Data::Struct(s) => visitor_for_struct(&name, s),
        _ => Err(MacroError::Message(
            "enable_duplicate_keys is only valid for structs",
            item.span(),
        )),
    }?;

    let deserializer = deserializer_for_struct(&name)?;

    let output = quote! {
        #[derive(EnableDuplicateKeys)]
        #original
        #visitor
        #deserializer
    }
    .into();
    Ok(output)
}

/// Denotes that the struct expects to read data that has multiple keys, and must be treated as such.
#[proc_macro_derive(EnableDuplicateKeys, attributes(from_duplicate_key))]
pub fn enable_duplicate_keys(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}

#[proc_macro_attribute]
pub fn duplicate_keys(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let result = duplicate_keys_impl(input);
    match result {
        Ok(stream) => stream,
        Err(err) => err.into_token_stream(),
    }
    .into()
}
