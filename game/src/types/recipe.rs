use crate::traits::identifiable::Identifiable;
use crate::types::localization::Localization;
use crate::types::resource::ResourceIO;
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

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: RecipeId,
    pub identifier: String,
    pub name: Localization,
    pub kind: RecipeKind,
    pub inputs: Vec<ResourceIO>,
    pub outputs: Vec<ResourceIO>,
}

impl Identifiable<String> for Recipe {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
