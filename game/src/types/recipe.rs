use crate::traits::identifiable::Identifiable;
use crate::types::resource::ResourceId;
use serde::{Deserialize, Serialize};

pub type RecipeIdSize = u16;
pub type RecipeIOSize = u8;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(pub RecipeIdSize);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RecipeIO {
    pub resource: ResourceId,
    pub amount: RecipeIOSize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub identifier: String,
    pub inputs: Vec<RecipeIO>,
    pub outputs: Vec<RecipeIO>,
}

impl Identifiable<String> for Recipe {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
