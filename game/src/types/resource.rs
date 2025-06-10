use crate::traits::identifiable::Identifiable;
use crate::types::localization::Localization;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ResourceIdSize = u16;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(pub ResourceIdSize);

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize,
)]
pub enum ResourceShape {
    #[default]
    Circle,
    Triangle,
    Square,
    Star,
}

#[derive(Debug, Default, Clone)]
pub struct ResourceIO {
    pub resource: Arc<Resource>,
    pub amount: u64,
}

impl ResourceIO {
    pub fn new(resource: Arc<Resource>, amount: u64) -> Self {
        Self { resource, amount }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Resource {
    pub id: ResourceId,
    pub identifier: String,
    pub name: Localization,
    pub shape: ResourceShape,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl Resource {
    pub fn get_name(&self, language: &str, fallback_language: &str) -> String {
        self.name
            .translate(language, fallback_language)
            .unwrap_or(self.identifier.clone())
    }
}

impl Identifiable<String> for Resource {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
}
