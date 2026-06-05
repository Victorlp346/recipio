use crate::{RecipioError, catalog::units::MeasurementCategory};

use async_trait::async_trait;
use bon::bon;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ingredient {
    id: Uuid,
    name: String,
    display_name: String,
    measure_category: MeasurementCategory,
}

#[bon]
impl Ingredient {
    #[builder(on(String, into))]
    pub fn new(
        id: Option<Uuid>,
        name: String,
        display_name: Option<String>,
        measure_category: MeasurementCategory,
    ) -> Self {
        let id = id.unwrap_or_else(Uuid::now_v7);

        let display_name = match display_name {
            None => name.clone(),
            Some(d_name) => d_name,
        };

        Ingredient {
            id,
            name,
            display_name,
            measure_category,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[async_trait]
pub trait IngredientRepository {
    async fn store_ingredient(&self, ingredient: &Ingredient) -> Result<(), RecipioError>;
    async fn get_ingredient(&self, id: &Uuid) -> Result<Option<Ingredient>, RecipioError>;
    async fn delete_ingredient(&self, id: &Uuid) -> Result<(), RecipioError>;
    async fn update_ingredient(
        &self,
        id: &Uuid,
        updated_ingredient: &Ingredient,
    ) -> Result<(), RecipioError>;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn builder_generates_uuid_if_missing() {
        let ingredient = Ingredient::builder()
            .name("Garlic")
            .measure_category(MeasurementCategory::Subjective)
            .build();

        assert!(!ingredient.id().is_nil())
    }

    #[test]
    fn builder_defaults_display_name_to_name() {
        let ingredient = Ingredient::builder()
            .name("Garlic")
            .measure_category(MeasurementCategory::Subjective)
            .build();

        assert_eq!(ingredient.display_name, ingredient.name)
    }

    #[test]
    fn builder_respects_explicit_overrides() {
        let ingredient = Ingredient::builder()
            .name("Garlic")
            .display_name("Garlic Display")
            .measure_category(MeasurementCategory::Weight)
            .id(Uuid::from_str("7569daf0-7828-4689-8338-9268fbf98d00").unwrap())
            .build();

        assert_eq!(ingredient.display_name, "Garlic Display");
        assert_eq!(
            ingredient.id,
            Uuid::from_str("7569daf0-7828-4689-8338-9268fbf98d00").unwrap()
        )
    }
}
