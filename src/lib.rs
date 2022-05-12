use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_NBUCKETS: usize = 1;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
    deleted: bool,
}
pub struct HashMap<K, V> {
    addresses: Vec<Option<Entry<K, V>>>,
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

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
    V: Default,
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
        let mut new_addresses: Vec<Option<Entry<K, V>>> = Vec::with_capacity(new_size);
        new_addresses.extend((0..new_size).map(|_| None));
        for elem in self.addresses.drain(..) {
            match elem {
                None => continue,
                Some(entry) => {
                    if entry.deleted {
                        continue;
                    }
                    // we need to find correct address for entry in new vector
                    let mut attempt = 0;
                    loop {
                        let mut hasher = DefaultHasher::new();
                        entry.key.hash(&mut hasher);
                        attempt.hash(&mut hasher);
                        let address = (hasher.finish() % new_addresses.len() as u64) as usize;
                        match new_addresses[address] {
                            Some(_) => {
                                attempt += 1;
                                continue;
                            }
                            None => {
                                new_addresses[address] = Some(entry);
                                break;
                            }
                        }
                    }
                }
            }
        }
        self.addresses = new_addresses;
    }
    fn find<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut attempt = 0;
        loop {
            match self.address(key, attempt) {
                None => return None,
                Some(address) => match self.addresses[address] {
                    None => return None,
                    Some(ref entry) => {
                        if entry.key.borrow() == key {
                            return Some(address);
                        }
                        attempt += 1;
                    }
                },
            }
        }
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
        match self.find(&key) {
            None => {
                // insert new, update size and return None
                let mut attempt = 0;
                loop {
                    let address = self
                        .address(&key, attempt)
                        .expect("is_empty, handled above");
                    match self.addresses[address] {
                        Some(_) => {
                            attempt += 1;
                            continue;
                        }
                        None => {
                            self.addresses[address] = Some(Entry {
                                key,
                                value,
                                deleted: false,
                            });
                            self.items += 1;
                            return None;
                        }
                    }
                }
            }
            Some(address) => {
                // key exists, check deleted flag, update, return old value if not deleted
                match self.addresses[address] {
                    None => None, // it will not be None here
                    Some(ref mut entry) => {
                        if entry.deleted {
                            entry.deleted = false;
                            _ = mem::replace(&mut entry.value, value);
                            self.items += 1;
                            return None;
                        }
                        Some(mem::replace(&mut entry.value, value))
                    }
                }
            }
        }
    }
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.find(key) {
            None => false,
            Some(address) => match self.addresses[address] {
                None => false,
                Some(ref entry) => !entry.deleted,
            },
        }
    }
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let address = self.find(key)?;
        match self.addresses[address] {
            Some(ref entry) => {
                if entry.deleted {
                    return None;
                }
                Some(&entry.value)
            }
            None => None,
        }
    }
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let address = self.find(key)?;
        match self.addresses[address] {
            None => None,
            Some(ref mut entry) => {
                if entry.deleted {
                    return None;
                }
                entry.deleted = true;
                let dv: V = Default::default();
                self.items -= 1;
                Some(mem::replace(&mut entry.value, dv))
            }
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
        map.insert("bar", 43);
        map.insert("foo", 42);
        assert_eq!(map.len(), 2);
        map.insert("bazz", 123);
        assert!(!map.is_empty());
        assert!(map.contains_key("bar"));
        assert!(map.contains_key("foo"));
        assert_eq!(map.get("foo"), Some(&42));
        assert_eq!(map.remove("foo"), Some(42));
        assert_eq!(map.get("foo"), None);
        assert_eq!(map.addresses.len(), 2)
    }
    #[test]
    fn empty_hashmap() {
        let mut map = HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("key"), false);
        assert_eq!(map.get("key"), None);
        assert_eq!(map.remove("key"), None);
    }
}
