use crate::data::parsing::resource::ResourceIdentifier;
use crate::types::recipe::{Recipe, RecipeIO, RecipeId, RecipeIdSize, RecipeKind};
use crate::types::resource::ResourceId;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub type RecipeIdentifier = String;

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct RawRecipeIO {
    pub resource: ResourceIdentifier,
    pub amount: u64,
}

impl RawRecipeIO {
    pub fn build(
        self,
        resource_dictionary: &HashMap<ResourceIdentifier, ResourceId>,
    ) -> Result<RecipeIO, Box<dyn Error>> {
        let resource = resource_dictionary.get(&self.resource).ok_or(format!(
            "Failed to parse recipe I/O, invalid resource identifier: '{}'",
            self.resource
        ))?;

        Ok(RecipeIO {
            resource: *resource,
            amount: self.amount,
        })
    }
}

#[derive(Encode, Decode, Serialize, Deserialize)]
pub struct RawRecipe {
    pub identifier: RecipeIdentifier,
    pub kind: RecipeKind,
    pub inputs: Vec<RawRecipeIO>,
    pub outputs: Vec<RawRecipeIO>,
}

impl RawRecipe {
    pub fn build(
        self,
        id: RecipeId,
        resource_dictionary: &HashMap<ResourceIdentifier, ResourceId>,
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

        let recipe = Recipe {
            id,
            identifier: self.identifier,
            kind: self.kind,
            inputs,
            outputs,
        };

        Ok(recipe)
    }
}

pub fn build_recipes(
    raw_recipes: Vec<RawRecipe>,
    resource_dictionary: &HashMap<ResourceIdentifier, ResourceId>,
) -> Result<HashMap<RecipeId, Recipe>, Box<dyn Error>> {
    let mut id_counter: RecipeIdSize = 0;
    raw_recipes
        .into_iter()
        .map(|raw_recipe| {
            let id = RecipeId(id_counter);
            id_counter += 1;
            let recipe = raw_recipe.build(id, resource_dictionary)?;
            Ok((id, recipe))
        })
        .collect::<Result<HashMap<_, _>, _>>()
}
