use crate::data::GameData;
use crate::types::node::NodeTrait;
use crate::types::resource::ResourceId;
use std::sync::Arc;

pub struct ResourceNode {
    pub resource: ResourceId,
    pub amount: u64,
}

impl NodeTrait for ResourceNode {
    fn title(&self, game_data: &Arc<GameData>) -> String {
        let Some(resource) = game_data.get_resource(self.resource) else {
            return "UNDEFINED".to_string();
        };
        resource.identifier.to_string()
    }
}
