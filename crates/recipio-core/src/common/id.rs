use derive_where::derive_where;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;

use crate::error::RecipioError;

#[derive_where(Clone, Debug, Eq, PartialEq, Hash, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Id<T> {
    pub fn new() -> Self {
        Self(Uuid::now_v7(), PhantomData)
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(raw: Uuid) -> Self {
        Self(raw, PhantomData)
    }
}

impl<T> FromStr for Id<T> {
    type Err = RecipioError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(raw).map_err(|_| RecipioError::ParsingError {
            value: raw.to_string(),
            target: String::from("uuid"),
        })?;
        Ok(Self(id, PhantomData))
    }
}
