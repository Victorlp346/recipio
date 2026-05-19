mod ingredient;
mod recipe;
mod units;
mod user;

use snafu::prelude::*;

pub use ingredient::{Ingredient, IngredientBuilder, IngredientRepository};

pub use units::{MeasurementCategory, SubjectiveUnit, VolumeUnit, WeightUnit};

#[derive(Debug, Snafu)]
pub enum RecipioError {
    #[snafu(display("An error ocurred on the repostory"))]
    RepositoryError,
}
