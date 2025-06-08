use crate::types::resource::{Resource, ResourceIO, ResourceId, ResourceIdSize, ResourceShape};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub type ResourceIdentifier = String;

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct RawResourceIO {
    pub resource: ResourceIdentifier,
    pub amount: u64,
}

impl RawResourceIO {
    pub fn build(
        self,
        resource_dictionary: &HashMap<ResourceIdentifier, Arc<Resource>>,
    ) -> Result<ResourceIO, Box<dyn Error>> {
        let resource = resource_dictionary.get(&self.resource).ok_or(format!(
            "Failed to parse recipe I/O, invalid resource identifier: '{}'",
            self.resource
        ))?;

        Ok(ResourceIO {
            resource: resource.clone(),
            amount: self.amount,
        })
    }
}

#[derive(Encode, Decode, Serialize, Deserialize)]
pub struct RawResource {
    pub identifier: ResourceIdentifier,
    pub shape: ResourceShape,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl RawResource {
    pub fn build(self, id: ResourceId) -> Resource {
        Resource {
            id,
            identifier: self.identifier,
            shape: self.shape,
            color_r: self.color_r,
            color_g: self.color_g,
            color_b: self.color_b,
        }
    }
}

pub fn build_resources(raw_resources: Vec<RawResource>) -> HashMap<ResourceId, Arc<Resource>> {
    let mut id_counter: ResourceIdSize = 0;
    raw_resources
        .into_iter()
        .map(|raw_resource| {
            let id = ResourceId(id_counter);
            id_counter += 1;
            (id, Arc::new(raw_resource.build(id)))
        })
        .collect()
}
