use crate::types::factory_node::resource::ResourceNode;
use crate::types::factory_node::smelter::SmelterNode;
use crate::types::recipe::Recipe;
use crate::types::resource::{Resource, ResourceIO};
use std::sync::Arc;

pub mod resource;
pub mod smelter;

pub trait FactoryNodeTrait {
    fn title(&self) -> String;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn input_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn output_at_index(&self, index: usize) -> Option<&ResourceIO>;
}

pub enum FactoryNode {
    Resource(ResourceNode),
    Smelter(SmelterNode),
}

impl FactoryNode {
    pub fn resource(resource: Arc<Resource>, amount: u64) -> Self {
        Self::Resource(ResourceNode::new(resource, amount))
    }

    pub fn smelter(recipe: Option<Arc<Recipe>>) -> Self {
        Self::Smelter(SmelterNode::new(recipe))
    }
}

impl FactoryNodeTrait for FactoryNode {
    fn title(&self) -> String {
        match self {
            Self::Resource(resource) => resource.title(),
            Self::Smelter(smelter) => smelter.title(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            Self::Resource(resource) => resource.inputs(),
            Self::Smelter(smelter) => smelter.inputs(),
        }
    }

    fn outputs(&self) -> usize {
        match self {
            Self::Resource(resource) => resource.outputs(),
            Self::Smelter(smelter) => smelter.outputs(),
        }
    }

    fn input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.input_at_index(index),
            Self::Smelter(smelter) => smelter.input_at_index(index),
        }
    }

    fn output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.output_at_index(index),
            Self::Smelter(smelter) => smelter.output_at_index(index),
        }
    }
}
