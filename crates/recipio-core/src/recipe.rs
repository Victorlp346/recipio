use bon::{bon, builder};
use uuid::Uuid;

use crate::units::MeasurementUnit;

pub struct Recipe {
    id: Uuid,
    name: String,
    steps: Vec<RecipeStep>,
    base_servings: u32,
}

#[bon]
impl Recipe {
    #[builder(on(String, into))]
    pub fn new(
        #[builder(field)] steps: Vec<RecipeStep>,
        id: Option<Uuid>,
        name: String,
        base_servings: u32,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(Uuid::now_v7),
            name,
            steps,
            base_servings,
        }
    }
}

impl<S: recipe_builder::State> RecipeBuilder<S> {
    fn step(mut self, step: RecipeStep) -> Self {
        self.steps.push(step);
        self
    }

    fn steps(mut self, steps: impl IntoIterator<Item: Into<RecipeStep>>) -> Self {
        self.steps.extend(steps.into_iter().map(Into::into));
        self
    }
}

pub struct RecipeStep {
    id: Uuid,
    step_number: u32,
    template_text: String,
    step_ingredients: Vec<StepIngredient>,
}

#[bon]
impl RecipeStep {
    #[builder]
    pub fn new(
        id: Option<Uuid>,
        step_number: u32,
        template_text: String,
        step_ingredients: Vec<StepIngredient>,
    ) -> Self {
        Self {
            id: id.unwrap_or(Uuid::now_v7()),
            step_number,
            template_text,
            step_ingredients,
        }
    }
}

pub struct StepIngredient {
    id: Uuid,
    ingredient_id: Uuid,
    amount: f64,
    unit: MeasurementUnit,
}

#[bon]
impl StepIngredient {
    #[builder]
    pub fn new(id: Option<Uuid>, ingredient_id: Uuid, amount: f64, unit: MeasurementUnit) -> Self {
        Self {
            id: id.unwrap_or(Uuid::now_v7()),
            ingredient_id,
            amount,
            unit,
        }
    }
}
