use proc_macro_error2::{ResultExt, abort};

pub(crate) struct FieldAttrsConfig {
    pub(crate) recursive: bool,
}

impl FieldAttrsConfig {
    pub fn parse_from_attrs(attrs: &[syn::Attribute]) -> Self {
        let mut recursive = false;

        for attr in attrs {
            if attr.path().is_ident("structful") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("recursive") {
                        recursive = true;
                        Ok(())
                    } else {
                        abort!(meta.path, "unrecognized flag");
                    }
                })
                .unwrap_or_abort();
            }
        }

        Self { recursive }
    }
}

pub(crate) struct FieldWithId<'a> {
    pub(crate) field: &'a syn::Field,
    pub(crate) id: proc_macro2::TokenStream,
}
