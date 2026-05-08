use crate::{RecipioError, units::MeasurementCategory};

use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ingredient {
    id: Uuid,
    name: String,
    display_name: String,
    measure_category: MeasurementCategory,
}

impl Ingredient {
    pub fn builder(name: String, measure_category: MeasurementCategory) -> IngredientBuilder {
        IngredientBuilder {
            name,
            measure_category,
            id: None,
            display_name: None,
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

#[derive(Default)]
pub struct IngredientBuilder {
    name: String,
    measure_category: MeasurementCategory,
    id: Option<Uuid>,
    display_name: Option<String>,
}

impl IngredientBuilder {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(String::from(display_name));
        self
    }

    pub fn build(self) -> Ingredient {
        let id = self.id.unwrap_or_else(Uuid::now_v7);

        let display_name = match self.display_name {
            None => self.name.clone(),
            Some(d_name) => d_name,
        };

        Ingredient {
            id,
            name: self.name,
            display_name,
            measure_category: self.measure_category,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_generates_uuid_if_missing() {
        let ingredient =
            Ingredient::builder(String::from("Garlic"), MeasurementCategory::Subjective).build();

        assert!(!ingredient.id().is_nil())
    }
}
