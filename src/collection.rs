use std::{hash::Hash, collections::HashMap};

pub struct Collection<K, V>{
    hashmap:HashMap<K,V>
}

impl<K,V> Default for Collection<K, V> {
    fn default() -> Self {
        Self {
            hashmap:HashMap::default()
        }
    }
}

impl<K,V> Collection<K, V>
where K : Copy + Clone + PartialEq + Eq + Hash {
    pub fn insert(&mut self, key:K, value:V) {
        self.hashmap.insert(key, value);
    }

    pub fn get(&self, key:&K) -> Option<&V> {
        self.hashmap.get(&key)
    }
}