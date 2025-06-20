use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Localization(HashMap<String, String>);

impl Localization {
    pub fn new(translations: HashMap<String, String>) -> Self {
        Self(translations)
    }

    pub fn get_localizations(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn translate(&self, language: &str, fallback: &str) -> Option<String> {
        if let Some(translation) = self.0.get(language) {
            Some(translation.to_string())
        } else {
            if let Some(translation) = self.0.get(fallback) {
                Some(translation.to_string())
            } else {
                None
            }
        }
    }
}
