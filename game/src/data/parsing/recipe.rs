use crate::data::parsing::localizations::RawLocalizations;
use crate::data::parsing::resource::{RawResourceIO, ResourceIdentifier};
use crate::types::recipe::{Recipe, RecipeId, RecipeIdSize, RecipeKind};
use crate::types::resource::Resource;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub type RecipeIdentifier = String;

#[derive(Encode, Decode, Serialize, Deserialize)]
pub struct RawRecipe {
    pub identifier: RecipeIdentifier,
    pub kind: RecipeKind,
    pub inputs: Vec<RawResourceIO>,
    pub outputs: Vec<RawResourceIO>,
}

impl RawRecipe {
    pub fn build(
        self,
        id: RecipeId,
        resource_dictionary: &HashMap<ResourceIdentifier, Arc<Resource>>,
        localizations: &RawLocalizations,
    ) -> Result<Recipe, Box<dyn Error>> {
        let inputs = self
            .inputs
            .into_iter()
            .map(|input| input.build(resource_dictionary))
            .collect::<Result<Vec<_>, _>>()?;

        let outputs = self
            .outputs
            .into_iter()
            .map(|output| output.build(resource_dictionary))
            .collect::<Result<Vec<_>, _>>()?;

        let name = localizations
            .recipe_names
            .get_localization(&self.identifier);

        let recipe = Recipe {
            id,
            identifier: self.identifier,
            name,
            kind: self.kind,
            inputs,
            outputs,
        };

        Ok(recipe)
    }
}

pub fn build_recipes(
    raw_recipes: Vec<RawRecipe>,
    resource_dictionary: &HashMap<ResourceIdentifier, Arc<Resource>>,
    localizations: &RawLocalizations,
) -> Result<HashMap<RecipeId, Arc<Recipe>>, Box<dyn Error>> {
    let mut id_counter: RecipeIdSize = 0;
    raw_recipes
        .into_iter()
        .map(|raw_recipe| {
            let id = RecipeId(id_counter);
            id_counter += 1;
            let recipe = raw_recipe.build(id, resource_dictionary, localizations)?;
            Ok((id, Arc::new(recipe)))
        })
        .collect::<Result<HashMap<_, _>, _>>()
}
