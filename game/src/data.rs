use crate::data::parsing::recipe::RecipeIdentifier;
use crate::data::parsing::resource::ResourceIdentifier;
use crate::types::recipe::{Recipe, RecipeId};
use crate::types::resource::{Resource, ResourceId};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::Arc;

pub mod parsing;

#[derive(Debug)]
pub struct GameData {
    pub default_language: String,
    pub languages: HashSet<String>,
    pub recipes_by_id: HashMap<RecipeId, Arc<Recipe>>,
    pub recipes_by_identifier: HashMap<RecipeIdentifier, Arc<Recipe>>,
    pub smelting_recipes: Vec<Arc<Recipe>>,
    pub resources_by_id: HashMap<ResourceId, Arc<Resource>>,
    pub resources_by_identifier: HashMap<ResourceIdentifier, Arc<Resource>>,
}

impl GameData {
    pub fn get_recipe(&self, id: RecipeId) -> Option<&Arc<Recipe>> {
        self.recipes_by_id.get(&id)
    }

    pub fn find_recipe<Q>(&self, identifier: &Q) -> Option<&Arc<Recipe>>
    where
        RecipeIdentifier: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.recipes_by_identifier.get(identifier)
    }

    pub fn get_resource(&self, id: ResourceId) -> Option<&Arc<Resource>> {
        self.resources_by_id.get(&id)
    }

    pub fn find_resource<Q>(&self, identifier: &Q) -> Option<&Arc<Resource>>
    where
        ResourceIdentifier: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.resources_by_identifier.get(identifier)
    }
}
