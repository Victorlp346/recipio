mod ingredient;
mod recipe;
mod units;
mod user;

use std::marker::PhantomData;
use std::str::FromStr;

use thiserror::Error;
use uuid::Uuid;

pub use ingredient::{Ingredient, IngredientBuilder, IngredientRepository};
pub use units::{MeasurementCategory, SubjectiveUnit, VolumeUnit, WeightUnit};

#[derive(Debug, Clone)]
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

#[derive(Debug, Error)]
pub enum RecipioError {
    #[error("repository error")]
    Repo(#[from] RepoError),
    #[error("impossible to parse {value:?} to {target:?}")]
    ParsingError { value: String, target: String },
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Unknown repository error")]
    UnknownError,
}

pub type RepoResult<T> = Result<T, RepoError>;
