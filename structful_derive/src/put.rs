use proc_macro_error2::abort_call_site;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Fields, Ident};

use crate::helpers::{FieldAttrsConfig, FieldWithId};

fn match_arms<'a>(fields: impl Iterator<Item = FieldWithId<'a>>) -> Vec<TokenStream> {
    fields.map(|field_decl| {
        let id = field_decl.id;
        let id_str = id.to_string();
        let ty = &field_decl.field.ty;
        let config = FieldAttrsConfig::parse_from_attrs(&field_decl.field.attrs);

        if config.leaf {
            quote! {
                Some(#id_str) => {
                    self.#id = <#ty as serde::Deserialize>::deserialize(deserializer)
                        .map_err(structful::Error::<D::Error>::from)?
                }
            }
        } else {
            quote! {
                Some(#id_str) => {
                    return structful::StructfulPut::structful_put(&mut self.#id, path, deserializer)
                }
            }
        }
    }).collect()
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
        impl<'de> StructfulPut<'de> for #name {
            fn structful_put<'a, P, D>(
                &mut self,
                mut path: P,
                deserializer: D,
            ) -> Result<(), Error<'a, D::Error>>
            where
                P: Iterator<Item = &'a str>,
                D: Deserializer<'de>,
            {
                match path.next() {
                    None => *self = Self::deserialize(deserializer).map_err(
                        structful::Error::<D::Error>::from
                    )?,
                    #(#match_arms),*,
                    Some(invalid) => return Err(structful::Error::invalid_path(
                        invalid,
                        ::std::format!(#err_fmt, invalid),
                    )),
                }
                Ok(())
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
