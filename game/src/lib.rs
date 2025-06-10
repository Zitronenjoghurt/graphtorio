use crate::data::parsing::RawGameData;
use crate::data::GameData;
use crate::types::factory::Factory;
use egui::Pos2;
use std::error::Error;
use std::sync::Arc;

pub mod data;
pub mod traits;
pub mod types;

pub struct Game {
    pub data: Arc<GameData>,
    pub factory: Factory,
}

impl Game {
    pub fn new(raw_game_data: RawGameData) -> Result<Self, Box<dyn Error>> {
        let data = raw_game_data.build()?;

        let game = Game {
            data: Arc::new(data),
            factory: Factory::default(),
        };

        Ok(game)
    }

    pub fn spawn_resource(&mut self, position: Pos2, identifier: &str, amount: u64) -> bool {
        let Some(resource) = self.data.find_resource(identifier) else {
            return false;
        };
        self.factory.spawn_resource(position, resource, amount);
        true
    }

    pub fn spawn_smelter(&mut self, position: Pos2, recipe_identifier: Option<&str>) -> bool {
        let Some(optional_recipe) =
            recipe_identifier.map(|identifier| self.data.find_recipe(identifier))
        else {
            return false;
        };
        self.factory.spawn_smelter(position, optional_recipe);
        true
    }
}
