use std::{
    borrow::Borrow,
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

    fn get_bucket<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
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

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_index = self.get_bucket(key)?;
        let bucket = &mut self.buckets[bucket_index];
        let index = bucket.iter().position(|(k, v)| k.borrow() == key)?;
        self.len -= 1;
        Some(bucket.swap_remove(index))
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_index = self.get_bucket(key)?;
        self.buckets[bucket_index]
            .iter()
            .find(|(ref k, _)| k.borrow() == key)
            .map(|(ref k, ref v)| v)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.at) {
                    Some(&(ref k, ref v)) => {
                        self.at += 1;
                        break Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
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

    #[test]
    fn iterator() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        map.insert("d", 4);

        for (&k, &v) in &map {
            match k {
                "a" => assert_eq!(v, 1),
                "b" => assert_eq!(v, 2),
                "c" => assert_eq!(v, 3),
                "d" => assert_eq!(v, 4),
                _ => unreachable!(),
            }
        }

        assert_eq!(map.len(), 4);
    }
}
