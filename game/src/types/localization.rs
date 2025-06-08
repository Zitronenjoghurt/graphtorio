use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Localization(HashMap<String, String>);

impl Localization {
    pub fn new(translations: HashMap<String, String>) -> Self {
        Self(translations)
    }

    pub fn translate(&self, language: &str, fallback: &str) -> String {
        if let Some(translation) = self.0.get(language) {
            translation.to_string()
        } else {
            if let Some(translation) = self.0.get(fallback) {
                translation.to_string()
            } else {
                "MISSING TRANSLATION".to_string()
            }
        }
    }
}
