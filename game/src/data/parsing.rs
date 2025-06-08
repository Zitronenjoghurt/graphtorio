use crate::data::parsing::config::RawConfig;
use crate::data::parsing::localizations::RawLocalizations;
use crate::data::parsing::recipe::{build_recipes, RawRecipe};
use crate::data::parsing::resource::{build_resources, RawResource};
use crate::data::GameData;
use crate::traits::identifiable::BuildIdentifierDictionary;
use crate::types::recipe::RecipeKind;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

mod config;
mod localizations;
pub mod recipe;
pub mod resource;

#[derive(Default, Encode, Decode, Serialize, Deserialize)]
pub struct RawGameData {
    recipes: Vec<RawRecipe>,
    resources: Vec<RawResource>,
    config: RawConfig,
    #[serde(default)]
    localizations: RawLocalizations,
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
        let resources = build_resources(self.resources, &self.localizations);
        let resource_dictionary = resources.build_identifier_dictionary();

        let recipes = build_recipes(self.recipes, &resource_dictionary, &self.localizations)?;
        let recipe_dictionary = recipes.build_identifier_dictionary();
        let smelting_recipes = recipes
            .values()
            .filter_map(|recipe| {
                if recipe.kind == RecipeKind::Smelting {
                    Some(recipe.clone())
                } else {
                    None
                }
            })
            .collect();

        let data = GameData {
            default_language: self.config.default_language,
            languages: self.localizations.get_languages(),
            recipes_by_id: recipes,
            recipes_by_identifier: recipe_dictionary,
            smelting_recipes,
            resources_by_id: resources,
            resources_by_identifier: resource_dictionary,
        };

        Ok(data)
    }
}
