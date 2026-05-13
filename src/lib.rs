#[cfg(test)]
mod tests;

use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Error<'a, E> {
    InvalidPath {
        component: &'a str,
        reason: Cow<'static, str>,
    },
    Serde(E),
}

impl<'a, E> From<E> for Error<'a, E> {
    fn from(value: E) -> Self {
        Self::Serde(value)
    }
}

impl<'a, E> Error<'a, E> {
    pub fn invalid_path(component: &'a str, reason: impl Into<Cow<'static, str>>) -> Self {
        Self::InvalidPath {
            component,
            reason: reason.into(),
        }
    }
}

pub trait StructfulGet: Serialize {
    fn structful_get<'a, P, S>(&self, path: P, serializer: S) -> Result<S::Ok, Error<'a, S::Error>>
    where
        P: Iterator<Item = &'a str>,
        S: Serializer;
}

pub trait StructfulPut<'de>: Deserialize<'de> {
    fn structful_put<'a, P, D>(
        &mut self,
        path: P,
        deserializer: D,
    ) -> Result<(), Error<'a, D::Error>>
    where
        P: Iterator<Item = &'a str>,
        D: Deserializer<'de>;
}

macro_rules! impl_without_fields {
    { $($type:ty),+ } => {
        $(
            impl StructfulGet for $type {
                fn structful_get<'a, P, S>(
                    &self,
                    mut path: P,
                    serializer: S,
                ) -> Result<S::Ok, Error<'a, S::Error>>
                where
                    P: Iterator<Item = &'a str>,
                    S: Serializer,
                {
                    match path.next() {
                        None => self.serialize(serializer).map_err(Error::from),
                        Some(invalid) => {
                            Err(Error::invalid_path(
                                invalid,
                                concat!(stringify!($type), " has no fields")
                            ))
                        },
                    }
                }
            }

            impl<'de> StructfulPut<'de> for $type {
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
                        None => {
                            *self =
                                <$type as serde::Deserialize>::deserialize(deserializer)
                                    .map_err(Error::from)?
                        }
                        Some(invalid) => {
                            return Err(Error::invalid_path(
                                invalid,
                                concat!(stringify!($type), " has no fields")
                            ));
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_without_fields! { u8, String }
