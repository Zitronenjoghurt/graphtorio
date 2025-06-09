use crate::types::factory_node::FactoryNodeTrait;
use crate::types::resource::{Resource, ResourceIO};
use std::sync::Arc;

pub struct ResourceNode {
    pub output: ResourceIO,
}

impl ResourceNode {
    pub fn new(resource: Arc<Resource>, amount: u64) -> Self {
        Self {
            output: ResourceIO { resource, amount },
        }
    }
}

impl FactoryNodeTrait for ResourceNode {
    fn title(&self) -> String {
        "Raw Resource".to_string()
    }

    fn inputs(&self) -> usize {
        0
    }

    fn outputs(&self) -> usize {
        1
    }

    fn input_at_index(&self, _index: usize) -> Option<&ResourceIO> {
        None
    }

    fn output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        if index == 0 { Some(&self.output) } else { None }
    }
}
