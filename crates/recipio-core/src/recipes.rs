use uuid::Uuid;

use crate::units::MeasurementUnit;

pub struct Recipe {
    id: Uuid,
    name: String,
    steps: Vec<RecipeStep>,
    base_servings: u32,
}

pub struct RecipeStep {
    id: Uuid,
    step_number: u32,
    template_text: String,
    step_ingredients: Vec<StepIngredient>,
}

pub struct StepIngredient {
    id: Uuid,
    ingredient_id: Uuid,
    amount: f64,
    unit: MeasurementUnit,
}
