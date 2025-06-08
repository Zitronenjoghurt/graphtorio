use crate::data::parsing::RawGameData;
use crate::data::GameData;
use std::error::Error;
use std::sync::Arc;

pub mod data;
pub mod traits;
pub mod types;

#[derive(Debug)]
pub struct Game {
    pub data: Arc<GameData>,
}

impl Game {
    pub fn new(raw_game_data: RawGameData) -> Result<Self, Box<dyn Error>> {
        let data = raw_game_data.build()?;

        let game = Game {
            data: Arc::new(data),
        };

        Ok(game)
    }
}
