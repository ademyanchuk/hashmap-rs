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
    Pair { key: K, val: V },
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
    K: Hash + Eq + Debug + Default,
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
            if let Entry::Pair { mut key, mut val } = entry {
                // rehash and add to new table
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let hash = hasher.finish();
                let mut idx = (hash % new_size as u64) as usize;
                while let Entry::Pair { key: _, val: _ } = new_table[idx] {
                    idx = (idx + 1) % new_size
                }
                let mut nk: K = Default::default();
                let mut nv: V = Default::default();
                mem::swap(&mut nk, &mut key);
                mem::swap(&mut nv, &mut val);
                new_table[idx] = Entry::Pair { key: nk, val: nv };
            }
        }
        self.table = new_table;
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
        let mut nv = value;
        let hash = self.prehash(&key);
        let mut idx = (hash % self.table.len() as u64) as usize;
        while !matches!(self.table[idx], Entry::Empty) {
            // pair
            if let Entry::Pair {
                key: ekey,
                val: eval,
            } = &mut self.table[idx]
            {
                if key.borrow() == ekey {
                    // existing key
                    mem::swap(eval, &mut nv);
                    return Some(nv);
                }
            }
            idx = (idx + 1) % self.table.len();
        }
        // new key
        self.table[idx] = Entry::Pair { key, val: nv };
        self.items += 1;
        None
    }
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug,
    {
        self.get(key).is_some()
    }
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug,
    {
        let hash = self.prehash(key);
        let mut idx = (hash % self.table.len() as u64) as usize;
        while !matches!(self.table[idx], Entry::Empty) {
            if let Entry::Pair { key: ek, val: ev } = &self.table[idx] {
                if ek.borrow() == key {
                    // found and return
                    return Some(ev);
                }
            }
            // linear probing
            idx = (idx + 1) % self.table.len();
        }
        // not found
        None
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
    fn resize() {
        let mut map = HashMap::<&str, &str>::new();
        assert!(map.table.is_empty());
        map.resize();
        map.resize();
        map.resize();
        assert_eq!(map.table.len(), INITIAL_SIZE * 4)
    }
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
        assert_eq!(map.get("barBazz"), None);
        assert!(!map.is_empty());
        assert!(map.contains_key("bazz"));
        assert!(map.contains_key("foo"));
        assert_eq!(map.get("foo"), Some(&42));
        // assert_eq!(map.remove("foo"), Some(42));
        // assert_eq!(map.get("foo"), None);
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
