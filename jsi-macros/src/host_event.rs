use inflector::Inflector;
use proc_macro2::TokenStream;

use quote::{format_ident, quote};
use syn::{parse::Parse, ItemEnum};

extern crate proc_macro;
pub struct HostEventImpl(pub TokenStream);

impl Parse for HostEventImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let enum_def: ItemEnum = input.parse()?;

        let enum_name = &enum_def.ident;

        let event_key_name = format_ident!("{}Key", enum_name);

        let mut event_names = vec![];
        let mut event_key_mappers = vec![];
        let mut event_args_mappers = vec![];

        for variant in &enum_def.variants {
            let variant_name = variant.ident.clone();
            event_names.push(variant_name.clone());

            event_key_mappers.push(match &variant.fields {
                syn::Fields::Named(_) => {
                    quote! { Self::#variant_name { .. } => #event_key_name::#variant_name }
                }
                syn::Fields::Unnamed(_) => {
                    quote! { Self::#variant_name ( .. ) => #event_key_name::#variant_name }
                }
                syn::Fields::Unit => {
                    quote! { Self::#variant_name => #event_key_name::#variant_name }
                }
            });

            event_args_mappers.push(match &variant.fields {
                syn::Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| f.ident.clone().unwrap())
                        .collect();

                    quote! { Self::#variant_name { #(#field_names),* } => {
                        let mut args: Vec<::jsi::JsiValue> = vec![];
                        #( args.push(::jsi::IntoValue::into_value(#field_names, rt)); )*
                        args
                    } }
                }
                syn::Fields::Unnamed(fields) => {
                    let field_names: Vec<_> = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| format_ident!("f{}", idx))
                        .collect();

                    quote! { Self::#variant_name { #(#field_names),* } => {
                        let mut args: Vec<::jsi::JsiValue> = vec![];
                        #( args.push(::jsi::IntoValue::into_value(#field_names, rt)); )*
                        args
                    } }
                }
                syn::Fields::Unit => {
                    quote! { Self::#variant_name => vec![], }
                }
            });
        }

        let event_key_def = quote! {
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            #[automatically_derived]
            pub enum #event_key_name {
                #(#event_names),*
            }
        };

        let event_names_str = event_names.iter().map(|id| id.to_string().to_camel_case());

        let event_key_impl_def = quote! {
            #[automatically_derived]
            impl ::std::str::FromStr for #event_key_name {
                type Err = ::anyhow::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(#event_names_str => Ok(Self::#event_names),)*
                        _ => bail!("invalid event name")
                    }
                }
            }

            #[automatically_derived]
            impl ::splicer_js_api::event::HostEventKey for #event_key_name {
            }
        };

        let has_rt_lifetime = enum_def.generics.params.iter().any(|p| match p {
            syn::GenericParam::Lifetime(lt) => lt.lifetime.ident == "rt",
            _ => false,
        });

        let enum_impl_generics = if has_rt_lifetime {
            quote! { <'rt> }
        } else {
            quote! {}
        };

        let event_impl_def = quote! {
            #[automatically_derived]
            impl<'rt> ::splicer_js_api::event::HostEvent<'rt> for #enum_name #enum_impl_generics {
                type Key = #event_key_name;

                fn key(&self) -> Self::Key {
                    match self {
                        #(#event_key_mappers),*
                    }
                }

                fn args(self, rt: &mut ::jsi::RuntimeHandle<'rt>) -> Vec<JsiValue<'rt>> {
                    match self {
                        #(#event_args_mappers),*
                    }
                }
            }
        };

        Ok(Self(quote! {
            #event_key_def

            #event_key_impl_def

            #event_impl_def
        }))
    }
}
