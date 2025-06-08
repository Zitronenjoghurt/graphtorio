use std::collections::HashMap;
use std::hash::Hash;

pub trait Identifiable<Id> {
    fn identifier(&self) -> Id;
}

pub trait BuildIdentifierDictionary<Id, K> {
    fn build_identifier_dictionary(&self) -> HashMap<Id, K>;
}

impl<Id, K, T> BuildIdentifierDictionary<Id, K> for HashMap<K, T>
where
    Id: Eq + Hash,
    K: Clone,
    T: Identifiable<Id>,
{
    fn build_identifier_dictionary(&self) -> HashMap<Id, K> {
        self.iter()
            .map(|(key, entry)| (entry.identifier(), key.clone()))
            .collect()
    }
}
