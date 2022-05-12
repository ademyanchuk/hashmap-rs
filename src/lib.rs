use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_SIZE: usize = 1;

#[derive(Debug)]
pub enum Entry<K, V> {
    Empty,
    Del,
    Pair(K, V),
}
pub struct HashMap<K, V> {
    table: Vec<Entry<K, V>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            table: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Default + Debug,
{
    fn prehash<Q>(&self, key: &Q) -> u64
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
    fn resize(&mut self) {
        let new_size = match self.table.len() {
            0 => INITIAL_SIZE,
            n => 2 * n,
        };
        let mut new_table: Vec<Entry<K, V>> = Vec::with_capacity(new_size);
        new_table.extend((0..new_size).map(|_| Entry::Empty));
        for entry in self.table.drain(..) {
            if let Entry::Pair(_, _) = entry {
                // rehash and add to new table
                let hash = self.prehash();
                let mut idx = (hash % new_size as u64) as usize;
                while let Entry::Pair(_, _) = new_table[idx] {
                    idx = (idx + 1) % new_size
                }
                new_table[idx] = entry;
            } else {
                continue;
            }
        }
        todo!()
    }
    pub fn len(&self) -> usize {
        self.items
    }
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }
        todo!()
    }
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug,
    {
        todo!()
    }
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug,
    {
        todo!()
    }
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("bar", 43);
        assert!(map.contains_key("bar"));
        map.insert("foo", 42);
        assert!(map.contains_key("bar"));
        assert_eq!(map.len(), 2);
        map.insert("bazz", 123);
        assert!(map.contains_key("foo"));
        println!("[test]: {:?}", map.table);
        assert_eq!(map.get("bar"), Some(&43));
        assert!(map.contains_key("bar"));
        assert!(!map.is_empty());
        assert!(map.contains_key("bazz"));
        assert!(map.contains_key("foo"));
        assert_eq!(map.get("foo"), Some(&42));
        assert_eq!(map.remove("foo"), Some(42));
        assert_eq!(map.get("foo"), None);
        assert_eq!(map.table.len(), 4)
    }
    #[test]
    fn empty_hashmap() {
        let mut map = HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("key"), false);
        assert_eq!(map.get("key"), None);
        assert_eq!(map.remove("key"), None);
    }
}
