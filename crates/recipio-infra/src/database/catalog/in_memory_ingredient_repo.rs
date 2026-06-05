use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use recipio_core::RecipioError;
use recipio_core::catalog::ingredient::{Ingredient, IngredientRepository};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryRepo {
    ingredients: Arc<RwLock<HashMap<Uuid, Ingredient>>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            ingredients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl IngredientRepository for InMemoryRepo {
    async fn store_ingredient(&self, ingredient: &Ingredient) -> Result<(), RecipioError> {
        self.ingredients
            .write()
            .await
            .insert(ingredient.id(), ingredient.clone());
        Ok(())
    }

    async fn get_ingredient(&self, id: &Uuid) -> Result<Option<Ingredient>, RecipioError> {
        let map = self.ingredients.read().await;
        Ok(map.get(id).cloned())
    }

    async fn delete_ingredient(&self, id: &Uuid) -> Result<(), RecipioError> {
        self.ingredients.write().await.remove(id);
        Ok(())
    }

    async fn update_ingredient(
        &self,
        id: &Uuid,
        updated_ingredient: &Ingredient,
    ) -> Result<(), RecipioError> {
        self.ingredients
            .write()
            .await
            .insert(*id, updated_ingredient.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_store_and_retrieve_ingredient() {
        let repo = InMemoryRepo::new();
        let ingredient = Ingredient::builder()
            .name("Garlic")
            .measure_category(recipio_core::catalog::units::MeasurementCategory::Weight)
            .build();

        assert!(repo.store_ingredient(&ingredient).await.is_ok());
        let retrieved_ingredient = repo
            .get_ingredient(&ingredient.id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(ingredient, retrieved_ingredient);
    }
}
