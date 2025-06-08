use crate::data::GameData;
use crate::types::node::resource::ResourceNode;
use crate::types::node::smelter::SmelterNode;
use std::sync::Arc;

pub mod resource;
pub mod smelter;

pub trait NodeTrait {
    fn title(&self, game_data: &Arc<GameData>) -> String;
}

pub enum Node {
    Resource(ResourceNode),
    Smelter(SmelterNode),
}

impl NodeTrait for Node {
    fn title(&self, game_data: &Arc<GameData>) -> String {
        match self {
            Self::Resource(resource) => resource.title(game_data),
            Self::Smelter(smelter) => smelter.title(game_data),
        }
    }
}
