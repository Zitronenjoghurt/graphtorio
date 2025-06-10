use crate::traits::identifiable::Identifiable;
use crate::types::localization::Localization;
use crate::types::resource::ResourceIO;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type RecipeIdSize = u16;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(pub RecipeIdSize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
pub enum RecipeKind {
    Assembling,
    Smelting,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: RecipeId,
    pub identifier: String,
    pub name: Localization,
    pub kind: RecipeKind,
    pub inputs: Vec<ResourceIO>,
    pub outputs: Vec<ResourceIO>,
}

impl Recipe {
    pub fn get_empty_inputs(&self) -> Vec<ResourceIO> {
        self.inputs
            .iter()
            .map(|io| ResourceIO::new(io.resource.clone(), 0))
            .collect()
    }

    pub fn get_empty_outputs(&self) -> Vec<ResourceIO> {
        self.outputs
            .iter()
            .map(|io| ResourceIO::new(io.resource.clone(), 0))
            .collect()
    }

    pub fn calculate_outputs(&self, inputs: &Vec<ResourceIO>) -> Vec<ResourceIO> {
        let mut smallest_ratio = 1.0f32;
        inputs.iter().enumerate().for_each(|(index, io)| {
            if !Arc::ptr_eq(&io.resource, &self.inputs[index].resource) {
                smallest_ratio = 0.0;
                return;
            }
            let ratio = io.amount as f32 / self.inputs[index].amount as f32;
            smallest_ratio = smallest_ratio.min(ratio);
        });

        self.outputs
            .iter()
            .map(|io| ResourceIO {
                resource: io.resource.clone(),
                amount: (io.amount as f32 * smallest_ratio) as u64,
            })
            .collect()
    }
}

impl Identifiable<String> for Recipe {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
