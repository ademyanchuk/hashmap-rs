use std::hash::Hash;

#[derive(Debug)]
pub enum Entry<K, V> {
    Empty,
    Deleted,
    Pair(K, V),
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
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
