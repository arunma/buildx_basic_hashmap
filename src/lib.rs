use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    mem,
};

const INITIAL_BUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.len > (3 * self.buckets.len() / 4) {
            self.resize_buckets();
        }

        let bucket_index = self
            .get_bucket(&key)
            .expect("Bucket is empty. That can't be");
        let bucket = &mut self.buckets[bucket_index];

        for (k, v) in bucket.iter_mut() {
            if &key == k {
                return Some(std::mem::replace(v, value));
            }
        }

        bucket.push((key, value));
        self.len += 1;
        None
    }

    fn get_bucket(&self, key: &K) -> Option<usize> {
        if self.buckets.is_empty() {
            None
        } else {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            Some((hasher.finish() % self.buckets.len() as u64) as usize)
        }
    }

    fn resize_buckets(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS,
            n => n * 2,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);

            let nbucket_index = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[nbucket_index].push((key, value));
        }

        mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        let bucket_index = self.get_bucket(key)?;
        let mut bucket = &mut self.buckets[bucket_index];
        let index = bucket.iter().position(|(k, v)| k == key)?;
        self.len -= 1;
        Some(bucket.swap_remove(index))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.get_bucket(key)?;
        self.buckets[bucket_index]
            .iter()
            .find(|(k, v)| k == key)
            .map(|(k, v)| v)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(0, map.len());
        assert!(map.is_empty());
        assert_eq!(map.contains_key(&"a"), false);

        map.insert("a", 1);
        assert_eq!(map.contains_key(&"a"), true);
        assert_eq!(1, map.len());
        assert!(!map.is_empty());
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        assert_eq!(1, map.len());
        assert!(!map.is_empty());
        map.remove(&"a");
        assert_eq!(map.contains_key(&"a"), false);
        assert_eq!(0, map.len());
        assert!(map.is_empty());
    }

    #[test]
    fn get() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        assert_eq!(map.get(&"a"), Some(&1));
        map.remove(&"a");
        assert_eq!(map.get(&"a"), None);
        assert_eq!(map.contains_key(&"a"), false);
    }
}