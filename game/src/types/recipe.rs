use crate::traits::identifiable::Identifiable;
use crate::types::resource::ResourceId;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type RecipeIdSize = u16;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(pub RecipeIdSize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
pub enum RecipeKind {
    Assembling,
    Smelting,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RecipeIO {
    pub resource: ResourceId,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub identifier: String,
    pub kind: RecipeKind,
    pub inputs: Vec<RecipeIO>,
    pub outputs: Vec<RecipeIO>,
}

impl Identifiable<String> for Recipe {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
