use crate::data::parsing::recipe::{build_recipes, RawRecipe};
use crate::data::parsing::resource::{build_resources, RawResource};
use crate::data::GameData;
use crate::traits::identifiable::BuildIdentifierDictionary;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

pub mod recipe;
pub mod resource;

#[derive(Encode, Decode, Serialize, Deserialize)]
pub struct RawGameData {
    recipes: Vec<RawRecipe>,
    resources: Vec<RawResource>,
}

impl RawGameData {
    pub fn decode(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(bincode::decode_from_slice(bytes, bincode::config::standard())?.0)
    }

    pub fn load_yaml(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn build(self) -> Result<GameData, Box<dyn Error>> {
        let resources = build_resources(self.resources);
        let resource_dictionary = resources.build_identifier_dictionary();

        let recipes = build_recipes(self.recipes, &resource_dictionary)?;
        let recipe_dictionary = recipes.build_identifier_dictionary();

        let data = GameData {
            recipes_by_id: recipes,
            recipes_by_identifier: recipe_dictionary,
            resources_by_id: resources,
            resources_by_identifier: resource_dictionary,
        };

        Ok(data)
    }
}
