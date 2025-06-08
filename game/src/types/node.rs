use crate::types::node::resource::ResourceNode;
use crate::types::node::smelter::SmelterNode;
use crate::types::recipe::Recipe;
use crate::types::resource::{Resource, ResourceIO};
use std::sync::Arc;

pub mod resource;
pub mod smelter;

pub trait NodeTrait {
    fn title(&self) -> String;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn input_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn output_at_index(&self, index: usize) -> Option<&ResourceIO>;
}

pub enum Node {
    Resource(ResourceNode),
    Smelter(SmelterNode),
}

impl Node {
    pub fn resource_node(resource: Arc<Resource>, amount: u64) -> Self {
        Self::Resource(ResourceNode::new(resource, amount))
    }

    pub fn smelter_node(recipe: Arc<Recipe>) -> Self {
        Self::Smelter(SmelterNode::new(recipe))
    }
}

impl NodeTrait for Node {
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
