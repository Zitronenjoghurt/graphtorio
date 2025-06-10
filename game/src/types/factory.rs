use crate::types::factory::node::FactoryNode;
use crate::types::recipe::Recipe;
use crate::types::resource::Resource;
use egui::Pos2;
use egui_snarl::Snarl;
use std::sync::Arc;

pub mod node;

#[derive(Default)]
pub struct Factory {
    pub snarl: Snarl<FactoryNode>,
}

impl Factory {
    pub fn spawn_resource(&mut self, position: Pos2, resource: &Arc<Resource>, amount: u64) {
        let node = FactoryNode::resource(resource.clone(), amount);
        self.snarl.insert_node(position, node);
    }

    pub fn spawn_smelter(&mut self, position: Pos2, optional_recipe: Option<&Arc<Recipe>>) {
        let node = FactoryNode::smelter(optional_recipe.cloned());
        self.snarl.insert_node(position, node);
    }
}
