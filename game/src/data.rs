use crate::types::recipe::{Recipe, RecipeId};
use crate::types::resource::{Resource, ResourceId};
use std::collections::HashMap;

pub mod parsing;

#[derive(Debug)]
pub struct GameData {
    pub recipes: HashMap<RecipeId, Recipe>,
    pub resources: HashMap<ResourceId, Resource>,
}
