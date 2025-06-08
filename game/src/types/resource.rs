use crate::traits::identifiable::Identifiable;
use serde::{Deserialize, Serialize};

pub type ResourceIdSize = u16;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(pub ResourceIdSize);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: ResourceId,
    pub identifier: String,
}

impl Identifiable<String> for Resource {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
