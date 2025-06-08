use crate::types::resource::{Resource, ResourceId, ResourceIdSize};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ResourceIdentifier = String;

#[derive(Encode, Decode, Serialize, Deserialize)]
pub struct RawResource {
    pub identifier: ResourceIdentifier,
}

impl RawResource {
    pub fn build(self, id: ResourceId) -> Resource {
        Resource {
            id,
            identifier: self.identifier,
        }
    }
}

pub fn build_resources(raw_resources: Vec<RawResource>) -> HashMap<ResourceId, Resource> {
    let mut id_counter: ResourceIdSize = 0;
    raw_resources
        .into_iter()
        .map(|raw_resource| {
            let id = ResourceId(id_counter);
            id_counter += 1;
            (id, raw_resource.build(id))
        })
        .collect()
}
