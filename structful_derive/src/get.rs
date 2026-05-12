use proc_macro_error2::abort_call_site;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Fields, Ident};

use crate::helpers::{FieldAttrsConfig, FieldWithId};

fn match_arms<'a>(fields: impl Iterator<Item = FieldWithId<'a>>) -> Vec<TokenStream> {
    fields
        .map(|field_decl| {
            let id = field_decl.id;
            let id_str = id.to_string();
            let config = FieldAttrsConfig::parse_from_attrs(&field_decl.field.attrs);

            if config.recursive {
                quote! {
                    Some(#id_str) => {
                        structful::StructfulGet::structful_get(&self.#id, path, serializer)
                    }
                }
            } else {
                quote! {
                    Some(#id_str) => {
                        serde::Serialize::serialize(&self.#id, serializer).map_err(|err| err.into())
                    }
                }
            }
        })
        .collect()
}

fn derive_struct(name: Ident, data: &syn::DataStruct) -> TokenStream {
    let match_arms = match data.fields {
        Fields::Named(ref fields) => match_arms(fields.named.iter().map(|field| {
            FieldWithId {
                field,
                id: field
                    .ident
                    .as_ref()
                    .expect("named fields have some ident")
                    .to_token_stream(),
            }
        })),

        Fields::Unit => match_arms(std::iter::empty()),

        Fields::Unnamed(ref fields) => match_arms(fields.unnamed.iter().enumerate().map(
            |(index, field)| FieldWithId {
                field,
                id: proc_macro2::Literal::usize_unsuffixed(index).to_token_stream(),
            },
        )),
    };

    let err_fmt = format!("no field `{{}}` on type `{name}`");

    TokenStream::from(quote! {
        impl structful::StructfulGet for #name {
            fn structful_get<'a, P, S>(
                &self,
                mut path: P,
                serializer: S,
            ) -> Result<S::Ok, structful::Error<'a, S::Error>>
            where
                P: Iterator<Item = &'a str>,
                S: Serializer,
            {
                match path.next() {
                    None => self.serialize(serializer).map_err(|err| err.into()),
                    #(#match_arms),*,
                    Some(invalid) => Err(structful::Error::invalid_path(
                        invalid,
                        format!(#err_fmt, invalid),
                    )),
                }
            }
        }
    })
}

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Struct(ref data) => derive_struct(input.ident, data),
        syn::Data::Enum(_) => abort_call_site!("not implemented for enums"),
        syn::Data::Union(_) => abort_call_site!("not implemented for unions"),
    }
}
