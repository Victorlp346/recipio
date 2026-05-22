pub mod error;
pub mod hasher;
pub mod ingredient;
pub mod recipe;
pub mod session;
pub mod units;
pub mod user;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{hash::Hash, marker::PhantomData};
use uuid::Uuid;

pub use error::{RecipioError, RecipioResult, RepoError, RepoResult};

#[derive(Debug)]
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

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid = Uuid::deserialize(deserializer)?;

        Ok(Id(uuid, PhantomData))
    }
}
