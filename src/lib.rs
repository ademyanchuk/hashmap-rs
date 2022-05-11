use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_NBUCKETS: usize = 1;

#[derive(Debug, Clone)]
pub enum Entry<K, V> {
    Empty,
    Deleted,
    Pair { key: K, value: V },
}
pub struct HashMap<K, V> {
    addresses: Vec<Entry<K, V>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            addresses: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn address<Q>(&self, key: &Q, attempt: i64) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.addresses.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        attempt.hash(&mut hasher);
        Some((hasher.finish() % self.addresses.len() as u64) as usize)
    }
    fn resize(&mut self) {
        let new_size = match self.addresses.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_addresses: Vec<Entry<K, V>> = Vec::with_capacity(new_size);
        new_addresses.extend((0..new_size).map(|_| Entry::Empty));
        // TODO: rehash entries from old vector to new
        _ = mem::replace(&mut self.addresses, new_addresses);
    }
    pub fn len(&self) -> usize {
        self.items
    }
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.addresses.is_empty() || self.items > 3 * self.addresses.len() / 4 {
            self.resize();
        }
        // probe underlying vector until either same key is found
        // or vector !Entry::Pair found
        // None if new key was inserted, otherwise old (replaced) value
        let mut attempt = 0;
        let mut address = self
            .address(&key, attempt)
            .expect(".is_empty() handled above");
        loop {
            match self.addresses[address] {
                // Empty = no entry with such key
                Entry::Empty => {
                    self.addresses[address] = Entry::Pair { key, value };
                    self.items += 1;
                    return None;
                }
                // If Pair found, check key, and if key is the same, update value
                Entry::Pair {
                    key: ref ekey,
                    value: ref mut evalue,
                } => {
                    if ekey == &key {
                        return Some(mem::replace(evalue, value));
                    }
                }
                // Deleted = continue searching
                Entry::Deleted => (),
            }
            attempt += 1;
            address = self
                .address(&key, attempt)
                .expect(".is_empty() handled above");
        }
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
        map.insert("key: K", 43);
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
    }
}
