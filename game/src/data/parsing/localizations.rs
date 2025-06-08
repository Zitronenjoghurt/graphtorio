use crate::types::localization::Localization;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Default, Encode, Decode, Serialize, Deserialize)]
pub struct RawLocalization(HashMap<String, HashMap<String, String>>);

impl RawLocalization {
    pub fn get_localization(&self, identifier: &str) -> Localization {
        let mut translations = HashMap::new();
        self.0
            .iter()
            .for_each(|(language, identifier_translations)| {
                if let Some(translation) = identifier_translations.get(identifier) {
                    translations.insert(language.to_lowercase(), translation.clone());
                }
            });
        Localization::new(translations)
    }

    pub fn get_languages(&self) -> HashSet<String> {
        self.0.keys().map(|s| s.to_string()).collect()
    }
}

#[derive(Default, Encode, Decode, Serialize, Deserialize)]
pub struct RawLocalizations {
    pub recipe_names: RawLocalization,
    pub resource_names: RawLocalization,
}

impl RawLocalizations {
    pub fn get_languages(&self) -> HashSet<String> {
        self.recipe_names
            .get_languages()
            .union(&self.resource_names.get_languages())
            .cloned()
            .collect()
    }
}
