use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Default, Encode, Decode, Serialize, Deserialize)]
pub struct RawConfig {
    pub default_language: String,
}
