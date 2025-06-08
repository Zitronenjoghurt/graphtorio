use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

pub trait Identifiable<Id> {
    fn identifier(&self) -> Id;
}

pub trait BuildIdentifierDictionary<Id, E> {
    fn build_identifier_dictionary(&self) -> HashMap<Id, Arc<E>>;
}

impl<Id, K, E> BuildIdentifierDictionary<Id, E> for HashMap<K, Arc<E>>
where
    Id: Eq + Hash,
    K: Clone,
    E: Identifiable<Id>,
{
    fn build_identifier_dictionary(&self) -> HashMap<Id, Arc<E>> {
        self.values().map(|e| (e.identifier(), e.clone())).collect()
    }
}
