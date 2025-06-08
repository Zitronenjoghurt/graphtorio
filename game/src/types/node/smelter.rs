use crate::data::GameData;
use crate::types::node::NodeTrait;
use crate::types::recipe::RecipeId;
use std::sync::Arc;

pub struct SmelterNode {
    pub recipe: RecipeId,
}

impl NodeTrait for SmelterNode {
    fn title(&self, game_data: &Arc<GameData>) -> String {
        let Some(recipe) = game_data.get_recipe(self.recipe) else {
            return "UNDEFINED".to_string();
        };
        recipe.identifier.to_string()
    }
}
