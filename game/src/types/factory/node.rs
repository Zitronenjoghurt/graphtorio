use crate::types::factory::node::resource::ResourceNode;
use crate::types::factory::node::smelter::SmelterNode;
use crate::types::recipe::Recipe;
use crate::types::resource::{Resource, ResourceIO};
use egui::ahash::HashMap;
use std::sync::Arc;

pub mod resource;
pub mod smelter;

pub trait FactoryNodeTrait {
    fn title(&self) -> String;
    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;
    fn true_input_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn desired_input_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn true_output_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn desired_output_at_index(&self, index: usize) -> Option<&ResourceIO>;
    fn process_inputs(&mut self, inputs: HashMap<usize, ResourceIO>);
    fn on_clear_io(&mut self) {}
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

    fn input_count(&self) -> usize {
        match self {
            Self::Resource(resource) => resource.input_count(),
            Self::Smelter(smelter) => smelter.input_count(),
        }
    }

    fn output_count(&self) -> usize {
        match self {
            Self::Resource(resource) => resource.output_count(),
            Self::Smelter(smelter) => smelter.output_count(),
        }
    }

    fn true_input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.true_input_at_index(index),
            Self::Smelter(smelter) => smelter.true_input_at_index(index),
        }
    }

    fn desired_input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.desired_input_at_index(index),
            Self::Smelter(smelter) => smelter.desired_input_at_index(index),
        }
    }

    fn true_output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.true_output_at_index(index),
            Self::Smelter(smelter) => smelter.true_output_at_index(index),
        }
    }

    fn desired_output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        match self {
            Self::Resource(resource) => resource.desired_output_at_index(index),
            Self::Smelter(smelter) => smelter.desired_output_at_index(index),
        }
    }

    fn process_inputs(&mut self, inputs: HashMap<usize, ResourceIO>) {
        match self {
            Self::Resource(resource) => resource.process_inputs(inputs),
            Self::Smelter(smelter) => smelter.process_inputs(inputs),
        }
    }

    fn on_clear_io(&mut self) {
        match self {
            Self::Resource(resource) => resource.on_clear_io(),
            Self::Smelter(smelter) => smelter.on_clear_io(),
        }
    }
}
