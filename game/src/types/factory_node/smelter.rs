use crate::types::factory_node::FactoryNodeTrait;
use crate::types::recipe::Recipe;
use crate::types::resource::ResourceIO;
use std::sync::Arc;

pub struct SmelterNode {
    pub recipe: Option<Arc<Recipe>>,
    pub selection_filter: String,
}

impl SmelterNode {
    pub fn new(recipe: Option<Arc<Recipe>>) -> Self {
        Self {
            recipe,
            selection_filter: String::new(),
        }
    }
}

impl FactoryNodeTrait for SmelterNode {
    fn title(&self) -> String {
        "Smelter".to_string()
    }

    fn inputs(&self) -> usize {
        if let Some(recipe) = &self.recipe {
            recipe.inputs.len()
        } else {
            0
        }
    }

    fn outputs(&self) -> usize {
        if let Some(recipe) = &self.recipe {
            recipe.outputs.len()
        } else {
            0
        }
    }

    fn input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        if let Some(recipe) = &self.recipe {
            recipe.inputs.get(index)
        } else {
            None
        }
    }

    fn output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        if let Some(recipe) = &self.recipe {
            recipe.outputs.get(index)
        } else {
            None
        }
    }
}
