use crate::types::node::NodeTrait;
use crate::types::recipe::Recipe;
use crate::types::resource::ResourceIO;
use std::sync::Arc;

pub struct SmelterNode {
    pub recipe: Arc<Recipe>,
}

impl SmelterNode {
    pub fn new(recipe: Arc<Recipe>) -> Self {
        Self { recipe }
    }
}

impl NodeTrait for SmelterNode {
    fn title(&self) -> String {
        "Smelter".to_string()
    }

    fn inputs(&self) -> usize {
        self.recipe.inputs.len()
    }

    fn outputs(&self) -> usize {
        self.recipe.outputs.len()
    }

    fn input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        self.recipe.inputs.get(index)
    }

    fn output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        self.recipe.outputs.get(index)
    }
}
