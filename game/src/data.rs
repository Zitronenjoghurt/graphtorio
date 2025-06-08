use crate::types::recipe::{Recipe, RecipeId};
use crate::types::resource::{Resource, ResourceId};
use std::collections::HashMap;

pub mod parsing;

#[derive(Debug)]
pub struct GameData {
    pub recipes: HashMap<RecipeId, Recipe>,
    pub resources: HashMap<ResourceId, Resource>,
}

impl GameData {
    pub fn get_recipe(&self, id: RecipeId) -> Option<&Recipe> {
        self.recipes.get(&id)
    }

    pub fn get_resource(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(&id)
    }
}
